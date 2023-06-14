use crate::regex_checker::*;
use crate::events::*;

use std::collections::HashMap;
use std::{
    io::*,
    str,
};

use subprocess::{
    Popen,
    PopenConfig,
    Redirection
};

use core::fmt::Arguments;

static mut P: Option<Popen> = None;

pub struct Player<'a> {
    pub name: &'a str,
    pub uuid: &'a str,
}

pub fn init(path: &str, xmx: &str, xms: &str, gui: &str) {
    println!("Initializing wrapper");
    
    unsafe {
        P = Some(Popen::create(&["java", xmx, xms, "-jar", path, match gui { "false" => "nogui", _ => "" }], PopenConfig {
            stdout: Redirection::Pipe, 
            stdin: Redirection::Pipe,
            ..Default::default()
        }).expect("failed to execute"));

        let mut uuids: HashMap<String,String> = HashMap::new();

        match &mut P {
            Some(c) => {
                if let Some(stdout) = &mut c.stdout {
                    let reader = BufReader::new(stdout);
                    let lines = reader.lines();
                    for line in lines {
                        let line = line.unwrap();
            
                        let line_type = get_type(&line);
            

                        // TODO: Use anything but this

                        if line_type == MessageType::AddUuid {
                            let playername = line.split("]: ").nth(1).unwrap().split("UUID of player ").nth(1).unwrap().split(" is").nth(0).unwrap().to_string();
                            let uuid = line.split(" is ").nth(1).unwrap().to_string();
                            uuids.insert(playername, uuid);

                        } else if line_type == MessageType::Join {
                            let name: &str = line.split("]: ").nth(1).unwrap().split("[").nth(0).unwrap().trim();
                            let uuid: &str = uuids.get(name).unwrap();

                            join_event::fire_event(Player {
                                name,
                                uuid
                            });
                            
                        } else if line_type == MessageType::Leave {
                            let name: &str = line.split("]: ").nth(1).unwrap().split(" lost connection: ").nth(0).unwrap();
                            let reason: &str = line.split("lost connection: ").nth(1).unwrap();
                            let uuid = uuids.get(name);
                            if uuid == None {
                                continue;
                            }

                            leave_event::fire_event(Player {
                                name,
                                uuid: uuid.unwrap()
                            }, reason);

                        } else if line_type == MessageType::Chat {
                            let name: &str = line.split("<").nth(1).unwrap().split(">").nth(0).unwrap();
                            let content: &str = line.split("> ").nth(1).unwrap();

                            chat_event::fire_event(Player {
                                name, uuid: uuids.get(name).unwrap() 
                            }, content);
                            
                        } else if line_type == MessageType::ServerStart {
                            start_event::fire_event();

                        } else if line_type == MessageType::ServerClose {
                            stop_event::fire_event();

                        }
            
                        println!("{}", line);
                    }
                }
            },
            None => println!("Child is null!"),
        }
    }
}


pub fn send_cmd(args: Arguments<>) {
    unsafe {
        match &mut P {
            Some(c) => {
                match &mut c.stdin {
                    Some(stdin) => {
                        stdin.write(format!("{:?}\n", args).as_bytes()).expect("failed to execute");
                    },
                    None => panic!("stdin is null!"),
                }
            },
            None => panic!("Child is null"),
        }
    }
}

#[macro_export]
macro_rules! send_cmd {
    () => {
        send_cmd("\n");
    };
    ($($arg:tt)*) => {{
        send_cmd(std::format_args!($($arg)*));
    }};
}
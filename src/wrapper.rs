use crate::regex_checker::*;
use crate::events::*;

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

static mut p: Option<Popen> = None;

pub fn init(path: &str) {
    println!("Initializing wrapper");
    
    unsafe {
        p = Some(Popen::create(&["java", "-jar", path], PopenConfig {
            stdout: Redirection::Pipe, 
            stdin: Redirection::Pipe,
            ..Default::default()
        }).expect("failed to execute"));

        match &mut p {
            Some(c) => {
                if let Some(stdout) = &mut c.stdout {
                    let reader = BufReader::new(stdout);
                    let lines = reader.lines();
                    for line in lines {
                        let line = line.unwrap();
            
                        let line_type = get_type(&line);
            
                        if line_type == MessageType::Join {
                            let playername = line.split("]: ").nth(1).unwrap().split(" joined the game").nth(0).unwrap();
                            join_event::fire_event(playername);
                        } else if line_type == MessageType::Leave {
                            let playername = line.split("]: ").nth(1).unwrap().split(" lost connection: ").nth(0).unwrap();
                            let reason = line.split("lost connection: ").nth(1).unwrap();
                            leave_event::fire_event(playername, reason);
                        } else if line_type == MessageType::Chat {
                            let playername = line.split("<").nth(1).unwrap().split(">").nth(0).unwrap();
                            let content = line.split("> ").nth(1).unwrap();
                            chat_event::fire_event(playername, content);
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
        match &mut p {
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
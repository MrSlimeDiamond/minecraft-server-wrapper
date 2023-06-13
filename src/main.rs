#[macro_use]
mod wrapper;
mod regex_checker;
mod events;

use std::{
    io::{self, Write},
    path::Path,
    thread,
    collections::HashMap, fs::File
};
use config::Config;
use wrapper::send_cmd;

fn main() {
    println!("Please wait...");

    let config = Config::builder()
        .add_source(config::File::with_name("wrapper_config"));

    let config = match config.build() {
        Ok(config) => config,
        Err(error) => match error.to_string().ends_with("not found") {
            true => {
                println!(r#"Creating "wrapper_config.ini" with default values, you may want to change these!"#);
                File::create("wrapper_config.ini").unwrap().write_all(b"jar_file = minecraft_server.jar\nxmx = -Xmx2G\nxms = -Xms2G").unwrap();
                Config::builder().add_source(config::File::with_name("wrapper_config")).build().unwrap()
            },
            false => panic!("Something went wrong with the config files")
        }
    };


    let config_data = config
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    let jar_file = config_data.get("jar_file"). expect("error getting jar_file from config");
    let xmx = config_data.get("xmx").           expect("error getting xmx from config");
    let xms = config_data.get("xms").           expect("error getting xms from config");

    println!(r#"Using JAR file: "{jar_file}""#);

    thread::spawn(|| {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("failed to read stdin");
            if buffer.trim() == "" {
                continue;
            }
            send_cmd!("{}", buffer.trim());
        }
    });

    if Path::new(jar_file).exists() {
        wrapper::init(jar_file, xmx, xms);
    } else {
        println!(r#"JAR file: "{jar_file}" was not found!"#);
        std::process::exit(1);
    }
}

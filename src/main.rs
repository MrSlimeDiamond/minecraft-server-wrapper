#[macro_use]
mod wrapper;
mod regex_checker;
mod events;

use std::{
    io,
    path::Path,
    thread,
    collections::HashMap
};
use config::Config;
use wrapper::send_cmd;

fn main() {
    println!("Please wait...");

    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();
    let config_data = config
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    let jar_file = config_data.get("jar_file").expect("error getting jar_file from config");
    let xmx = config_data.get("xmx").expect("error getting xmx from config");
    let xms = config_data.get("xms").expect("error getting xms from config");

    println!("Using jar file: {}", jar_file);

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
        println!("Minecraft jar was not found!");
        println!("Does it have the correct file name?");
        println!("Expected: {jar_file}");
        std::process::exit(1);
    }
}

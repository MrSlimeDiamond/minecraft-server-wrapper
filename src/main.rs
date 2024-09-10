#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
mod wrapper;
mod regex_checker;
mod events;
mod api;

use std::{
    io::{self, Write},
    path::Path,
    collections::HashMap,
    fs::File,
    thread
};
use config::Config;
use wrapper::send_cmd;
use api::network;

#[tokio::main]
async fn main() {
    println!("Please wait...");

    let config = Config::builder()
        .add_source(config::File::with_name("wrapper_config"));

    let config = match config.build() {
        Ok(config) => config,
        Err(error) => match error.to_string().ends_with("not found") {
            true => {
                println!(r#"Creating "wrapper_config.ini" with default values, you may want to change these!"#);
                File::create("wrapper_config.ini").unwrap().write_all(
                    b"java_exec = java\njar_file = minecraft_server.jar\ngui = true\nxmx = -Xmx2G\nxms = -Xms2G\napi-port = 7867\napi-key = supersecretkey").unwrap();
                Config::builder().add_source(config::File::with_name("wrapper_config")).build().unwrap()
            },
            false => panic!("Something went wrong with the config file")
        }
    };

    let config_data = config
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    let java_exec = config_data.get("java_exec").       expect("error: java_exec does not exist in wrapper_config")
    let jar_file = config_data.get("jar_file").         expect("error: jar_file value does not exist in wrapper_config");
    let gui = config_data.get("gui").                   expect("error: gui value does not exist in wrapper_config");
    let xmx = config_data.get("xmx").                   expect("error: xmx value does not exist in wrapper_config");
    let xms = config_data.get("xms").                   expect("error: xms value does not exist in wrapper_config");
    config_data.get("api-key").                         expect("error: api-key value does not exist in wrapper_config");
    let _port: u16 = config_data.get("api-port").       expect("error: api-port value does not exist in wrapper_config").parse().expect("error: api-port is not a valid number");

    println!(r#"Using JAR file: "{jar_file}""#);

    tokio::spawn(async {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("failed to read stdin");
            if buffer.trim() == "" {
                continue;
            }
            send_cmd!("{}", buffer.trim());
        }
    });

    if Path::new(&jar_file).exists() {
        wrapper::init(jar_file, xmx, xms, gui).await;

        tokio::task::spawn_blocking(|| {
            network::main();
        }).await.expect("API panicked")
    } else {
        println!(r#"JAR file: "{jar_file}" was not found!"#);
        std::process::exit(1);
    }
}
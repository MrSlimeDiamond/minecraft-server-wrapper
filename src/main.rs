#[macro_use]
pub mod wrapper;
pub use wrapper::*;
pub mod regex_checker;
pub use regex_checker::*;
pub mod events;
pub use events::*;

use std::{
    io,
    path::Path,
    thread
};

static JAR_PATH: &str = "minecraft_server.jar";
fn main() {
    println!("Please wait...");

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

    if Path::new(JAR_PATH).exists() {
        wrapper::init(JAR_PATH);
    } else {
        println!("Jar file could not be found!");
        println!("Ensure that the Minecraft jar has the name:");
        println!("{JAR_PATH}");
        std::process::exit(1);
    }
}

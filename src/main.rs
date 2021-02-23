mod get_info;
mod update_daemon;

use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};

fn main() {
    let server = TcpListener::bind("127.0.0.1:15744").unwrap();
    println!("Welcome to the Brunch Tools Daemon!");
    if !whoami::username().eq("root") {
        println!("Warning: You're not running as the root user, this may cause issues.");
    }
    if get_info::get_brunch_ver().eq("NONE") {
        println!("Warning: You're not running this on a system with brunch, this may cause issues.");
    }
    println!("Note: Outside of debugging or development purposes, this should only be executed by the init service file.");
    println!("Running on {}", server.local_addr().unwrap().to_string());
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            println!("New client connected from {}", websocket.get_ref().peer_addr().unwrap().to_string());
            loop {
                let message = websocket.read_message().unwrap();

                if message.is_text() {
                    let parsed = json::parse(message.into_text().unwrap().as_str()).unwrap();

                    if parsed["action"].is_null() || parsed["action"].is_empty() {
                        websocket.write_message(Message::from("{\"error\":\"UNEXPECTED INPUT\"}")).unwrap();
                        continue;
                    }
                    match parsed["action"].as_str().unwrap() {
                        "get_info" => websocket.write_message(get_info::get_info()).unwrap(),
                        "update_daemon" => update_daemon::update_daemon(),
                        _ => websocket.write_message(Message::from("{\"error\":\"UNEXPECTED INPUT\"}")).unwrap()
                    }
                }
            }
        });
    }
}
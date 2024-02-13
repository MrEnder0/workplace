use std::thread;

use tungstenite::{connect, Message};
use url::Url;

use crate::STATUS;

pub fn client() {
    'connection: loop {
        let (mut socket, _) = match connect(
            Url::parse(format!("ws://{}:3012/socket", get_server_ip()).as_str()).unwrap(),
        ) {
            Ok((socket, response)) => (socket, response),
            Err(_) => {
                println!("Can't connect, retrying in 5 seconds...");
                thread::sleep(std::time::Duration::from_secs(5));
                continue 'connection;
            }
        };

        let mut id = -1;

        println!("Connected to the server");

        loop {
            match socket.read() {
                Ok(Message::Binary(bin)) => match bin[0] {
                    0 => {
                        if id == -1 {
                            let uuid = String::from_utf8(bin[1..].to_vec()).unwrap();
                            id = uuid.parse::<i32>().unwrap();
                        }
                    }
                    1 => {
                        if id != -1 {
                            socket
                                .send(Message::Text(format!("HeartBeat:{}", id)))
                                .unwrap();
                        }
                    }
                    2 => {
                        if !*STATUS.lock().unwrap() {
                            *STATUS.lock().unwrap() = true;
                        }
                    }
                    3 => {
                        if *STATUS.lock().unwrap() {
                            *STATUS.lock().unwrap() = false;
                        }
                    }
                    4 => {
                        let request_id = String::from_utf8(bin[1..].to_vec()).unwrap();
                        if request_id == id.to_string() {}
                    }
                    _ => println!("Received unknown message"),
                },
                // Used to handle server close
                Ok(Message::Close(_)) => {
                    break;
                }
                Ok(_) => continue,
                Err(_) => {
                    println!("Error reading message, server may have disconnected. Attempting to reconnect...");
                    break;
                }
            };
        }
    }
}

fn get_server_ip() -> String {
    match std::fs::read_to_string("server_ip") {
        Ok(file) => file,
        Err(_) => {
            println!("server_ip file not found, using localhost, please create server_ip file");
            "localhost".to_string()
        }
    }
}

use std::thread;

use tungstenite::{connect, Message};
use url::Url;

use crate::STATUS;

pub fn client() {
    'connection: loop {
        let (mut socket, _) = match connect(Url::parse("ws://localhost:3012/socket").unwrap()) {
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
                            println!("Received id: {}", id);
                        }
                    }
                    1 => {
                        if !*STATUS.lock().unwrap() {
                            println!("Received Allow message");
                            *STATUS.lock().unwrap() = true;
                        }
                    }
                    2 => {
                        if *STATUS.lock().unwrap() {
                            println!("Received Deny message");
                            *STATUS.lock().unwrap() = false;
                        }
                    }
                    3 => {
                        let request_id = String::from_utf8(bin[1..].to_vec()).unwrap();
                        if request_id == id.to_string() {
                            println!("Received Shutdown message");
                        }
                    }
                    _ => println!("Received unknown message"),
                },
                // Used to handle server close
                Ok(Message::Close(_)) => {
                    println!("Received Close message");
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

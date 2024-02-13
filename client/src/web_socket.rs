use std::thread;

use tungstenite::{connect, Message};
use url::Url;
use workplace_common::{decode_server_packet, ClientAction, ServerAction};

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

        let mut id: Option<u8> = None;

        println!("Connected to the server");

        loop {
            match socket.read() {
                Ok(Message::Binary(bin)) => {
                    let decoded_packet = decode_server_packet(bin);

                    match decoded_packet {
                        ServerAction::Init(info) => {
                            id = Some(info.id);
                            println!(
                                "Received id {} from a server running version {}",
                                info.id, info.server_version
                            );
                        }
                        ServerAction::HeartBeat => {
                            socket
                                .send(Message::Binary(
                                    ClientAction::HeartBeat(id.unwrap()).into_bytes(),
                                ))
                                .unwrap();
                        }
                        ServerAction::Allow => {
                            if *STATUS.lock().unwrap() {
                                *STATUS.lock().unwrap() = false;
                            }
                        }
                        ServerAction::Deny => {
                            if !*STATUS.lock().unwrap() {
                                *STATUS.lock().unwrap() = true;
                            }
                        }
                        ServerAction::Shutdown(requested_id) => {
                            if requested_id == id.unwrap() {
                                println!("Server has requested a shutdown");
                                break;
                            }
                        }
                    }
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

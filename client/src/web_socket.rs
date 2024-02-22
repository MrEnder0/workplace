use std::thread;

use scorched::*;
use std::sync::atomic::Ordering;
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
                log_this(LogData {
                    importance: LogImportance::Warning,
                    message: "Failed to connect to server, retrying in 5 seconds".to_string(),
                });
                thread::sleep(std::time::Duration::from_secs(5));
                continue 'connection;
            }
        };

        let mut id: Option<u8> = None;

        log_this(LogData {
            importance: LogImportance::Info,
            message: "Connected to server".to_string(),
        });

        loop {
            match socket.read() {
                Ok(Message::Binary(bin)) => {
                    let decoded_packet = decode_server_packet(bin);

                    match decoded_packet {
                        ServerAction::Init(info) => {
                            id = Some(info.id);

                            if info.server_version != env!("CARGO_PKG_VERSION") {
                                log_this(LogData {
                                    importance: LogImportance::Warning,
                                    message:
                                        "Client version does not match server version, updating..."
                                            .to_string(),
                                });
                                crate::update_client(&info.server_version);
                            }
                        }
                        ServerAction::HeartBeat => {
                            socket
                                .send(Message::Binary(
                                    ClientAction::HeartBeat(id.unwrap()).into_bytes(),
                                ))
                                .unwrap();
                        }
                        ServerAction::Allow => {
                            STATUS.store(false, Ordering::Relaxed);
                        }
                        ServerAction::Deny => {
                            STATUS.store(true, Ordering::Relaxed);
                        }
                        ServerAction::Shutdown(requested_id) => {
                            if requested_id == id.unwrap() {
                                log_this(LogData {
                                    importance: LogImportance::Info,
                                    message: "Shutting down client...".to_string(),
                                });
                                std::process::Command::new("shutdown")
                                    .args(["/s", "/t", "0"])
                                    .spawn()
                                    .unwrap();
                            }
                        }
                        ServerAction::Restart(requested_id) => {
                            if requested_id == id.unwrap() {
                                log_this(LogData {
                                    importance: LogImportance::Info,
                                    message: "Restarting client...".to_string(),
                                });
                                std::process::Command::new("shutdown")
                                    .args(["/r", "/t", "0"])
                                    .spawn()
                                    .unwrap();
                            }
                        }
                    }
                }
                Ok(_) => continue,
                Err(_) => {
                    log_this(LogData {
                        importance: LogImportance::Warning,
                        message: "Server has disconected, attempting to reconnect...".to_string(),
                    });
                    break;
                }
            };
        }
    }
}

fn get_server_ip() -> String {
    match std::fs::read_to_string("C:/WorkPlace/server_ip.dat") {
        Ok(file) => file,
        Err(_) => {
            log_this(LogData {
                importance: LogImportance::Warning,
                message: "Failed to read server_ip.dat, using default IP".to_string(),
            });
            "localhost".to_string()
        }
    }
}

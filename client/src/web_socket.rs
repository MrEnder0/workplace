use std::thread;

use scorched::*;
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
                                update_client(&info.server_version);
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
                        message: "Error reading message, server may have disconnected. Attempting to reconnect...".to_string(),
                    });
                    break;
                }
            };
        }
    }
}

fn get_server_ip() -> String {
    match std::fs::read_to_string(r"C:\ProgramData\server_ip.dat") {
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

fn update_client(version: &str) {
    let url = format!(
        "https://github.com/MrEnder0/workplace/releases/download/{}/workplace-client.exe",
        version
    );
    let response = reqwest::blocking::get(url).unwrap();

    std::fs::write("workplace-client.exe.update", response.bytes().unwrap()).unwrap();

    self_replace::self_replace("workplace-client.exe.update").unwrap();
}

use crate::heartbeat;

use std::{collections::HashMap, net::TcpListener, thread::spawn};

use eframe::epaint::mutex::Mutex;
use once_cell::sync::Lazy;
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
    Message,
};
use workplace_common::{decode_client_packet, ClientAction, InitInfo, ServerAction};

static PENDING_ACTIONS: Lazy<Mutex<HashMap<u8, UiAction>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Copy)]
pub enum UiAction {
    Shutdown,
    Restart,
}

pub fn server() {
    loop {
        let server = TcpListener::bind("0.0.0.0:3012").unwrap();
        for stream in server.incoming() {
            spawn(move || {
                let callback = |_req: &Request, mut response: Response| {
                    println!("Received a new ws handshake");

                    let headers = response.headers_mut();
                    headers.append("MyCustomHeader", ":)".parse().unwrap());
                    headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

                    Ok(response)
                };
                let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

                websocket
                    .send(tungstenite::Message::binary(
                        ServerAction::Init(InitInfo {
                            id: heartbeat::assign_lowest_available_id(),
                            server_version: env!("CARGO_PKG_VERSION").to_string(),
                        })
                        .into_bytes(),
                    ))
                    .unwrap();

                spawn(move || loop {
                    match *crate::STATUS.lock().unwrap() {
                        true => {
                            websocket
                                .send(tungstenite::Message::binary(
                                    ServerAction::Deny.into_bytes(),
                                ))
                                .unwrap_or_else(|_| {
                                    println!("Failed to send deny message");
                                    let _ = websocket.close(None);
                                });
                        }
                        false => {
                            websocket
                                .send(tungstenite::Message::binary(
                                    ServerAction::Allow.into_bytes(),
                                ))
                                .unwrap_or_else(|_| {
                                    println!("Failed to send allow message");
                                    let _ = websocket.close(None);
                                });
                        }
                    }

                    websocket
                        .send(tungstenite::Message::binary(
                            ServerAction::HeartBeat.into_bytes(),
                        ))
                        .unwrap_or_else(|_| {
                            println!("Failed to send heartbeat message");
                            let _ = websocket.close(None);
                        });

                    match websocket.read() {
                        Ok(Message::Binary(bin)) => {
                            let decoded_packet = decode_client_packet(bin);

                            match decoded_packet {
                                ClientAction::HeartBeat(id) => {
                                    println!("Received heartbeat from id {}", id);
                                    heartbeat::update_heartbeat(id);
                                }
                            }
                        }
                        Ok(_) => {}
                        Err(_) => {
                            println!("Error reading message, server may have disconnected. Attempting to reconnect...");
                            break;
                        }
                    };

                    let pending_actions = PENDING_ACTIONS.lock().clone();

                    for (id, action) in pending_actions.iter() {
                        match action {
                            UiAction::Shutdown => {
                                websocket
                                    .send(tungstenite::Message::binary(
                                        ServerAction::Shutdown(*id).into_bytes(),
                                    ))
                                    .unwrap();
                            }
                            UiAction::Restart => {
                                websocket
                                    .send(tungstenite::Message::binary(
                                        ServerAction::Restart(*id).into_bytes(),
                                    ))
                                    .unwrap();
                            }
                        }
                    }

                    std::thread::sleep(std::time::Duration::from_secs(3));
                });
            });
        }
    }
}

pub fn request_shutdown(id: u8) {
    PENDING_ACTIONS.lock().insert(id, UiAction::Shutdown);
}

pub fn request_restart(id: u8) {
    PENDING_ACTIONS.lock().insert(id, UiAction::Restart);
}

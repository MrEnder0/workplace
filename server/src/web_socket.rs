use crate::{actions::ServerAction, heartbeat};

use std::{net::TcpListener, thread::spawn};

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

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
                        ServerAction::Init(heartbeat::assign_lowest_available_id().to_string()).into_bytes(),
                    ))
                    .unwrap();

                spawn(move || loop {
                    match *crate::STATUS.lock().unwrap() {
                        true => {
                            websocket
                                .send(tungstenite::Message::binary(ServerAction::Deny.into_bytes()))
                                .unwrap_or_else(|_| {
                                    println!("Failed to send deny message");
                                    let _ = websocket.close(None);
                                });
                        }
                        false => {
                            websocket
                                .send(tungstenite::Message::binary(ServerAction::Allow.into_bytes()))
                                .unwrap_or_else(|_| {
                                    println!("Failed to send allow message");
                                    let _ = websocket.close(None);
                                });
                        }
                    }

                    websocket
                        .send(tungstenite::Message::binary(ServerAction::HeartBeat.into_bytes()))
                        .unwrap();

                        let heartbeat = websocket.read().unwrap();

                    if heartbeat.is_text() {
                        let heartbeat = heartbeat.to_text().unwrap();
                        if heartbeat.starts_with("HeartBeat:") {
                            let id = heartbeat.split(':').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
                            println!("Received heartbeat from id {}", id);
                            heartbeat::update_heartbeat(id);
                        }
                    }

                    std::thread::sleep(std::time::Duration::from_secs(5));
                });
            });
        }
    }
}

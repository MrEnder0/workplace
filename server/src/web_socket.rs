use crate::actions::Action;

use std::{net::TcpListener, sync::Mutex, thread::spawn};

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

static ID: Mutex<u64> = Mutex::new(0);

pub fn server() {
    loop {
        let server = TcpListener::bind(format!("{}:3012", get_server_ip())).unwrap();
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
                        Action::Init((*ID.lock().unwrap()).to_string()).into_bytes(),
                    ))
                    .unwrap();
                *ID.lock().unwrap() += 1;

                spawn(move || loop {
                    match *crate::STATUS.lock().unwrap() {
                        true => {
                            websocket
                                .send(tungstenite::Message::binary(Action::Deny.into_bytes()))
                                .unwrap();
                        }
                        false => {
                            websocket
                                .send(tungstenite::Message::binary(Action::Allow.into_bytes()))
                                .unwrap();
                        }
                    }

                    std::thread::sleep(std::time::Duration::from_secs(5));
                });
            });
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

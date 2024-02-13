pub enum ServerAction {
    /// Gives unassigned clients their id, id will have client specific interactions in the future
    Init(InitInfo),
    /// Request to allow the clients to reply with their id to show they are still active
    HeartBeat,
    /// Notifys the client that it is allowed
    Allow,
    /// Notifys the client that it is denied
    Deny,
    /// Shutdown a client based on the id given by the init action, planned use in the future
    Shutdown(String),
}

pub struct InitInfo {
    pub id: u8,
    pub server_version: String,
}

impl ServerAction {
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            ServerAction::Init(input) => {
                let mut bytes = vec![0];
                bytes.push(input.id);
                bytes.extend(input.server_version.as_bytes());
                bytes
            }
            ServerAction::HeartBeat => vec![1],
            ServerAction::Allow => vec![2],
            ServerAction::Deny => vec![3],
            ServerAction::Shutdown(id) => {
                let mut bytes = vec![4];
                bytes.push(id.parse::<u8>().unwrap());
                bytes
            }
        }
    }
}

pub enum ClientAction {
    /// Reply to the server's heartbeat request with the client's id
    HeartBeat(u8),
}

impl ClientAction {
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            ClientAction::HeartBeat(id) => {
                let mut bytes = vec![0];
                bytes.push(id);
                bytes
            }
        }
    }
}

pub fn decode_server_packet(packet: Vec<u8>) -> ServerAction {
    match packet[0] {
        0 => {
            let id = packet[1];
            let server_version = String::from_utf8(packet[2..].to_vec()).unwrap();
            ServerAction::Init(InitInfo { id, server_version })
        }
        1 => ServerAction::HeartBeat,
        2 => ServerAction::Allow,
        3 => ServerAction::Deny,
        4 => {
            let id = String::from_utf8(packet[1..].to_vec()).unwrap();
            ServerAction::Shutdown(id)
        }
        _ => panic!("Unknown action"),
    }
}

pub fn decode_client_packet(packet: Vec<u8>) -> ClientAction {
    match packet[0] {
        0 => {
            let id = packet[1];
            ClientAction::HeartBeat(id)
        }
        _ => panic!("Unknown action"),
    }
}

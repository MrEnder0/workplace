pub enum ServerAction {
    /// Gives unassigned clients their id, id will have client specific interactions in the future
    Init(String),
    /// Request to allow the clients to reply with their id to show they are still active
    HeartBeat,
    /// Notifys the client that it is allowed
    Allow,
    /// Notifys the client that it is denied
    Deny,
    /// Shutdown a client based on the id given by the init action, planned use in the future
    Shutdown(String),
}

impl ServerAction {
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            ServerAction::Init(id) => {
                let mut bytes = vec![0];
                bytes.extend_from_slice(id.as_bytes());
                bytes
            }
            ServerAction::HeartBeat => vec![1],
            ServerAction::Allow => vec![2],
            ServerAction::Deny => vec![3],
            ServerAction::Shutdown(id) => {
                let mut bytes = vec![4];
                bytes.extend_from_slice(id.as_bytes());
                bytes
            }
        }
    }
}

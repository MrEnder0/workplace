pub enum Action {
    /// Gives unassigned clients their id, id will have client specific interactions in the future
    Init(String),
    /// Notifys the client that it is allowed
    Allow,
    /// Notifys the client that it is denied
    Deny,
    /// Shutdown a client based on the id given by the init action, planned use in the future
    Shutdown(String),
}

impl Action {
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Action::Init(id) => {
                let mut bytes = vec![0];
                bytes.extend_from_slice(id.as_bytes());
                bytes
            }
            Action::Allow => vec![1],
            Action::Deny => vec![2],
            Action::Shutdown(id) => {
                let mut bytes = vec![3];
                bytes.extend_from_slice(id.as_bytes());
                bytes
            }
        }
    }
}

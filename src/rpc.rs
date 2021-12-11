pub enum Rpc {
    Ping,
    FindValue(String),
    FindNode(crate::key::Key),
    Store(String, String),
}


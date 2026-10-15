pub type MessagePoint = String;

pub struct MessageOut<OutMessage> {
    pub name: OutMessage,
    pub message: MessagePoint,
}

pub struct ConnectionSuccess {
    pub is_connected: bool,
}

pub trait Connect {
    fn Connected() -> Self;
}

impl Connect for ConnectionSuccess {
    fn Connected() -> Self {
        let self_connected = ConnectionSuccess { is_connected: true };

        self_connected
    }
}

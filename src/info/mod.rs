pub type MessagePoint = String;

#[derive(sqlx::FromRow)]

pub struct MessageOut {
    pub name: String,
    pub message: MessagePoint,
}

pub struct Success {
    pub is_connected: bool,
}

pub trait Connect {
    fn connected() -> Self;
}

impl Connect for Success {
    fn connected() -> Self {
        let self_connected = Success { is_connected: true };

        self_connected
    }
}

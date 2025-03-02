pub enum TransferAction {
    Send,
    Receive,
}

impl ToString for TransferAction {
    fn to_string(&self) -> String {
        match self {
            Self::Receive => String::from("receive"),
            Self::Send => String::from("send"),
        }
    }
}

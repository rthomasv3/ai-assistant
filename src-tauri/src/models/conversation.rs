use serde::Deserialize;
use serde::Serialize;
use crate::models::message::Message;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub messages: Vec<Message>
}

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub is_user: bool,
    pub text: String
}

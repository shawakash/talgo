use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SubMsg {
    #[serde(rename = "type")]
    pub message_type: String,
    pub channel: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Msg {
    #[serde(rename = "type")]
    pub message_type: String,
    pub channel: String,
    pub data: Option<String>,
}

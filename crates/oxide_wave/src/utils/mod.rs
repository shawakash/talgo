use data_dust::types::msg::Msg;

pub fn create_msg(message_type: &str, channels: &Vec<String>, data: Option<&str>) -> String {
    let msg = Msg {
        message_type: message_type.to_string(),
        channels: channels.iter().map(|s| s.to_string()).collect(),
        data: data.map(|s| s.to_string()),
    };
    serde_json::to_string(&msg).expect("Failed while serializing message")
}

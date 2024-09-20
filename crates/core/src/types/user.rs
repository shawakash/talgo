use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginReq {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRes {
    pub success: bool,
    pub message: String,
    pub user_id: Option<String>,
}

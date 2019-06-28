use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgotPassword {
    pub username: String
}

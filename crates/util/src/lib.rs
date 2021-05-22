use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LinkRequest {
    pub id: String,
    pub passcode: String,
    pub url: String,
}

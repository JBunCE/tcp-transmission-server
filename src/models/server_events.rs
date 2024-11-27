use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct StreamEvent {
    pub uuid: String,
    pub user_id: String,
    pub access_token: String,
}
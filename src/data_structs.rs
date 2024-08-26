use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgDate {
    id: u64,
    reaction: u64,
    msg: String,
    jailed: bool,
}

use crate::protocol::Message;
use serde_json;
use anyhow::Result;

/// Serialize a message to JSON
pub fn serialize(msg: &Message) -> Result<Vec<u8>> {
    let s = serde_json::to_vec(msg)?;
    Ok(s)
}

/// Deserialize a message from JSON
pub fn deserialize(data: &[u8]) -> Result<Message> {
    let msg = serde_json::from_slice(data)?;
    Ok(msg)
}

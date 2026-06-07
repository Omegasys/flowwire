use crate::protocol::Message;
use crate::message::{serialize, deserialize};
use crate::socket::{send, receive};
use std::os::unix::net::UnixStream;
use anyhow::Result;

/// Simple RPC over IPC
pub fn call(stream: &mut UnixStream, msg: &Message) -> Result<Message> {
    let data = serialize(msg)?;
    send(stream, &data)?;
    let response_data = receive(stream)?;
    let response = deserialize(&response_data)?;
    Ok(response)
}

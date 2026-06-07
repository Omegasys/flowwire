/// Defines message types for FlowWire IPC
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    RegisterNode { name: String, node_type: String },
    UpdateEdge { from: String, to: String },
    StreamData { stream_id: u32, data: Vec<u8> },
    QueryTopology,
    PluginCommand { name: String, command: String },
    Heartbeat,
}

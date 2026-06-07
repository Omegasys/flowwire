use flowwire_core::{Node, NodeType};
use anyhow::Result;

/// Trait all FlowWire plugins must implement
pub trait FlowWirePlugin {
    fn name(&self) -> &str;
    fn init(&self) -> Result<Vec<Node>>;
    fn shutdown(&self) -> Result<()> { Ok(()) }
    fn tick(&self) -> Result<()> { Ok(()) }
}

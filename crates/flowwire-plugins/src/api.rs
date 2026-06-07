use flowwire_core::{Node, NodeType};
use anyhow::Result;

/// Trait that all FlowWire plugins must implement
pub trait FlowWirePlugin {
    /// Name of the plugin
    fn name(&self) -> &str;

    /// Initialize the plugin (create nodes, register callbacks, etc.)
    fn init(&self) -> Result<Vec<Node>>;

    /// Optional: shutdown hook
    fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    /// Optional: called on every processing tick
    fn tick(&self) -> Result<()> {
        Ok(())
    }
}

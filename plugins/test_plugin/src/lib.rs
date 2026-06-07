use flowwire_plugins::api::FlowWirePlugin;
use flowwire_core::{Node, NodeType};
use anyhow::Result;

pub struct TestPlugin;

impl FlowWirePlugin for TestPlugin {
    fn name(&self) -> &str { "TestPlugin" }

    fn init(&self) -> Result<Vec<Node>> {
        let node = Node::new("VirtualEffect", NodeType::AudioOutput);
        Ok(vec![node])
    }
}

#[no_mangle]
pub extern "C" fn _create_plugin() -> *mut dyn FlowWirePlugin {
    Box::into_raw(Box::new(TestPlugin)) as *mut dyn FlowWirePlugin
}

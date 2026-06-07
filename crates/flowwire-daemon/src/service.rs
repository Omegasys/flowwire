use crate::config::Config;
use crate::discovery::discover_devices;
use crate::routing::RoutingEngine;
use anyhow::Result;
use flowwire_core::Graph;

/// Core daemon structure
pub struct FlowWireDaemon {
    pub config: Config,
    pub graph: Graph,
    pub routing: RoutingEngine,
}

impl FlowWireDaemon {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        let graph = Graph::new();
        let routing = RoutingEngine::new();
        Ok(Self { config, graph, routing })
    }

    pub fn run(&mut self) -> Result<()> {
        discover_devices(&mut self.graph)?;
        self.routing.start(&mut self.graph)?;
        println!("FlowWire daemon running...");
        // Keep running
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

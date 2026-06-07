use crate::loader::Plugin;
use anyhow::Result;

/// Manages loaded plugins
pub struct PluginManager {
    plugins: Vec<Plugin>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn register(&mut self, plugin: Plugin) -> Result<()> {
        let nodes = plugin.instance.init()?;
        log::info!("Loaded plugin '{}' with {} nodes", plugin.instance.name(), nodes.len());
        self.plugins.push(plugin);
        Ok(())
    }

    pub fn tick(&self) -> Result<()> {
        for plugin in &self.plugins {
            plugin.instance.tick()?;
        }
        Ok(())
    }

    pub fn shutdown(&self) -> Result<()> {
        for plugin in &self.plugins {
            plugin.instance.shutdown()?;
        }
        Ok(())
    }
}

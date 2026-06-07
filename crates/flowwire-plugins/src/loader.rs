use libloading::{Library, Symbol};
use anyhow::Result;
use crate::api::FlowWirePlugin;
use std::path::PathBuf;

/// Represents a dynamically loaded plugin
pub struct Plugin {
    pub lib: Library,
    pub instance: Box<dyn FlowWirePlugin + Send + Sync>,
}

pub struct Loader;

impl Loader {
    pub fn load(path: PathBuf) -> Result<Plugin> {
        unsafe {
            let lib = Library::new(&path)?;
            let constructor: Symbol<unsafe fn() -> *mut dyn FlowWirePlugin> =
                lib.get(b"_create_plugin")?;
            let boxed_raw = constructor();
            let instance = Box::from_raw(boxed_raw);
            Ok(Plugin { lib, instance })
        }
    }
}

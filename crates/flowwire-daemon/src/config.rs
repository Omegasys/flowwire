use anyhow::Result;

/// Stub configuration
#[derive(Debug)]
pub struct Config {
    pub ipc_socket_path: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        Ok(Self {
            ipc_socket_path: "/tmp/flowwire.sock".to_string(),
        })
    }
}

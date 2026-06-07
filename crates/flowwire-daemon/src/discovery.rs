use flowwire_core::Graph;
use anyhow::Result;

/// Fake device discovery (audio/video/midi)
pub fn discover_devices(_graph: &mut Graph) -> Result<()> {
    println!("Discovering devices...");
    // TODO: populate Graph with actual system devices
    Ok(())
}

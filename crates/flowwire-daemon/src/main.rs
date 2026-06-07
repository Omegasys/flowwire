use anyhow::Result;
use flowwire_daemon::service::FlowWireDaemon;

fn main() -> Result<()> {
    env_logger::init();
    println!("Starting FlowWire daemon...");

    let mut daemon = FlowWireDaemon::new()?;
    daemon.run()?;

    Ok(())
}

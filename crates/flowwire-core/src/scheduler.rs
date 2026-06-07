/// Stub scheduler for nodes
pub struct Scheduler;

impl Scheduler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn schedule(&self) {
        // TODO: Implement real-time scheduling for nodes
        println!("Scheduler tick");
    }
}

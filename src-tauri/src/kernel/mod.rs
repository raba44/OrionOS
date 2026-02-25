pub mod core;
pub mod memory;

use std::sync::Mutex;

pub struct Kernel {
    id: String,
    version: String,
    status: KernelStatus,
}

#[derive(Debug, Clone)]
pub enum KernelStatus {
    Initializing,
    Running,
    Paused,
    Shutting,
}

impl Kernel {
    pub fn new() -> Self {
        Kernel {
            id: uuid::Uuid::new_v4().to_string(),
            version: "0.1.0".to_string(),
            status: KernelStatus::Initializing,
        }
    }

    pub fn start(&mut self) {
        self.status = KernelStatus::Running;
        println!("Kernel started successfully!");
    }

    pub fn get_status(&self) -> KernelStatus {
        self.status.clone()
    }
}
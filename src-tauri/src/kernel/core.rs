pub struct CoreProcessor {
    clock_speed: u32,
    cores: u8,
}

impl CoreProcessor {
    pub fn new(cores: u8, clock_speed: u32) -> Self {
        CoreProcessor {
            clock_speed,
            cores,
        }
    }

    pub fn execute_task(&self, task_id: String) -> bool {
        println!("Executing task: {} on {} cores at {} MHz", task_id, self.cores, self.clock_speed);
        true
    }

    pub fn get_info(&self) -> String {
        format!("CPU: {} cores @ {} MHz", self.cores, self.clock_speed)
    }
}
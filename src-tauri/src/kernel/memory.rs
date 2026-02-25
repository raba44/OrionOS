use std::collections::HashMap;

pub struct MemoryManager {
    total_memory: u64,
    allocated: u64,
    memory_map: HashMap<String, u64>,
}

impl MemoryManager {
    pub fn new(total_memory: u64) -> Self {
        MemoryManager {
            total_memory,
            allocated: 0,
            memory_map: HashMap::new(),
        }
    }

    pub fn allocate(&mut self, process_id: String, size: u64) -> bool {
        if self.allocated + size <= self.total_memory {
            self.allocated += size;
            self.memory_map.insert(process_id, size);
            true
        } else {
            false
        }
    }

    pub fn deallocate(&mut self, process_id: &str) -> bool {
        if let Some(size) = self.memory_map.remove(process_id) {
            self.allocated -= size;
            true
        } else {
            false
        }
    }

    pub fn get_available_memory(&self) -> u64 {
        self.total_memory - self.allocated
    }
}
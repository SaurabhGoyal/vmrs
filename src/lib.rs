const MEMORY_SLOTS: usize = 1 << 12;

#[derive(Debug)]
pub struct Machine {
    registers: [u16; 8],
    memory: [u16; MEMORY_SLOTS],
}

impl Machine {
    pub fn step(&mut self) -> Result<(), String> {
        todo!()
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            registers: [0; 8],
            memory: [0; MEMORY_SLOTS],
        }
    }
}

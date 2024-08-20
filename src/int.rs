use std::collections::HashMap;

#[derive(Debug)]
pub struct InterruptController {
    interrupts: HashMap<u8, u8>,
}

impl InterruptController {
    pub fn new() -> Self {
        Self {
            interrupts: HashMap::new(),
        }
    }

    pub fn int(&mut self, dev: u8, int: u8) -> anyhow::Result<()> {
        self.interrupts.insert(int, dev);
        Ok(())
    }

    pub fn int_ack(&mut self, int: u8) -> anyhow::Result<()> {
        self.interrupts.remove(&int);
        Ok(())
    }
}

impl Default for InterruptController {
    fn default() -> Self {
        Self::new()
    }
}

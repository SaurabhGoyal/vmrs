const MEMORY_SLOT_COUNT: usize = 1 << 12;

const R0: usize = 0;
const R1: usize = 1;
const R2: usize = 2;
const R3: usize = 3;
const R4: usize = 4;
const R5: usize = 5;
const R6: usize = 6;
const R7: usize = 7;
const PC: usize = 8;
const RCOND: usize = 9;
const REG_COUNT: usize = 10;

trait Addressable {
    fn read(&self, addr: usize, slot_count: usize) -> anyhow::Result<&[u16]>;
    fn write(&mut self, addr: usize, values: &[u16]) -> anyhow::Result<()>;
    fn dump(&self) -> &[u16];
}

#[derive(Debug)]
struct Memory {
    slots: [u16; MEMORY_SLOT_COUNT],
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            slots: [0; MEMORY_SLOT_COUNT],
        }
    }
}

impl Addressable for Memory {
    fn read(&self, addr: usize, slot_count: usize) -> anyhow::Result<&[u16]> {
        if addr + slot_count > self.slots.len() {
            anyhow::bail!("Invalid address and slot count");
        }
        Ok(self.slots[addr..slot_count].as_ref())
    }

    fn write(&mut self, addr: usize, values: &[u16]) -> anyhow::Result<()> {
        if addr + values.len() > self.slots.len() {
            anyhow::bail!("Invalid value size for given address");
        }
        self.slots[addr..values.len()].copy_from_slice(values);
        Ok(())
    }

    fn dump(&self) -> &[u16] {
        self.slots.as_slice()
    }
}

pub struct Machine {
    registers: [u16; REG_COUNT],
    memory: Box<dyn Addressable>,
}

#[derive(Debug)]
pub struct Dump {
    registers: [u16; REG_COUNT],
    memory: Vec<u16>,
}

impl Machine {
    pub fn step(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    pub fn dump(&self) -> Dump {
        Dump {
            registers: self.registers,
            memory: self.memory.dump().to_owned(),
        }
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            registers: [0; REG_COUNT],
            memory: Box::new(Memory::default()),
        }
    }
}

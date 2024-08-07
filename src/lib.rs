const MEMORY_SLOT_COUNT: usize = 1 << 12;

const R0: usize = 0;
const R1: usize = 1;
const R2: usize = 2;
const R3: usize = 3;
const R4: usize = 4;
const R5: usize = 5;
const R6: usize = 6;
const R7: usize = 7;
const RPC: usize = 8;
const RCOND: usize = 9;
const REG_COUNT: usize = 10;

const OP_ADD: u16 = 1;
const OP_LOAD: u16 = 2;
const OP_TRAP: u16 = 15;

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
        Ok(self.slots[addr..addr + slot_count].as_ref())
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
    pub registers: [u16; REG_COUNT],
    pub memory: Vec<u16>,
}

impl Machine {
    pub fn run(&mut self, addr: u16, program_code: &[u16]) -> anyhow::Result<()> {
        self.memory.write(addr as usize, program_code)?;
        self.registers[RPC] = addr;
        while let Some(pc) = self.step()? {
            self.registers[RPC] = pc;
        }
        Ok(())
    }

    // Instruction format [OP_CODE(4 bit), Parameters (12 bit)];
    fn step(&mut self) -> anyhow::Result<Option<u16>> {
        let pc = self.registers[RPC];
        let instr = self.memory.read(pc as usize, 1)?[0];
        let op_code = instr >> 12;
        match op_code {
            OP_ADD => self.add(instr)?,
            OP_LOAD => self.load(instr)?,
            OP_TRAP => return Ok(None),
            _ => {}
        }
        Ok(Some(pc + 1))
    }

    pub fn dump(&self) -> Dump {
        Dump {
            registers: self.registers,
            memory: self.memory.dump().to_owned(),
        }
    }

    // Instruction formats-
    // Register mode - [OP_CODE(4 bit), Dest Register (3 bit), Source-Register-1 (3 bit), 000, Source-Register-2 (3 bit)];
    // Immediate mode - [OP_CODE(4 bit), Dest Register (3 bit), Source-Register-1 (3 bit), 1, Value (5 bit)];
    fn add(&mut self, instr: u16) -> anyhow::Result<()> {
        let dest_reg = ((instr >> 9) & 7) as usize;
        let source_reg_1 = ((instr >> 6) & 7) as usize;
        let mode = ((instr >> 5) & 1) as usize;
        let data = if mode == 1 {
            instr & ((1 << 5) - 1)
        } else {
            let source_reg_2 = (instr & 7) as usize;
            self.registers[source_reg_2]
        };
        self.registers[dest_reg] = self.registers[source_reg_1] + data;
        Ok(())
    }

    // Instruction format [OP_CODE(4 bit), Dest Register (3 bit), 0, Value (8 bit)];
    fn load(&mut self, instr: u16) -> anyhow::Result<()> {
        let reg = ((instr >> 9) & 7) as usize;
        self.registers[reg] = instr & 15;
        Ok(())
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

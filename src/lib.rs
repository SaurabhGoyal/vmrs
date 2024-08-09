const MEMORY_SLOT_COUNT: usize = 1 << 12;

// `R0`-`R7` are data storage registers. They are modelled as unsigned int16 but machine itself doesn't
// assign any numeric schemantic to them. For machine, they are 16-bit data storage which may store
// anything in it.
const R0: usize = 0;
const R1: usize = 1;
const R2: usize = 2;
const R3: usize = 3;
const R4: usize = 4;
const R5: usize = 5;
const R6: usize = 6;
const R7: usize = 7;
// `RPC` is a dedicated register to store which next instruction is to be executed. This enables non-linear
// execution of code which is powered by `go-to / jump` statement enabling connstructs such as `if-else` and `loop`.
const RPC: usize = 8;
// `RSTAT` is a dedicated register for things such as sign of last result (+ve / -ve), status of last operation
// (underflow / overflow), augmented information of last result (carry) and various interrupts. This is mostly
// used in the context of a branching decision as a quick lookup for deciding factors.
const RSTAT: usize = 9;

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
    pub registers: [i16; REG_COUNT],
    pub memory: Vec<i16>,
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
            registers: self.registers.map(|n| n as i16),
            memory: self
                .memory
                .dump()
                .to_owned()
                .iter()
                .map(|n| *n as i16)
                .collect::<Vec<i16>>(),
        }
    }

    // Instruction formats-
    // Register mode - [OP_CODE(4 bit), Dest Register (3 bit), Source-Register-1 (3 bit), 000, Source-Register-2 (3 bit)];
    // Immediate mode - [OP_CODE(4 bit), Dest Register (3 bit), Source-Register-1 (3 bit), 1, Value-Sign (1-bit), Value-Number (4 bit)];
    fn add(&mut self, instr: u16) -> anyhow::Result<()> {
        let dest_reg = ((instr >> 9) & 7) as usize;
        let source_reg_1 = ((instr >> 6) & 7) as usize;
        let mode = ((instr >> 5) & 1) as usize;
        let data = get_sign_extended_value(
            if mode == 1 {
                instr & ((1 << 5) - 1)
            } else {
                let source_reg_2 = (instr & 7) as usize;
                self.registers[source_reg_2]
            },
            5,
        );
        self.registers[dest_reg] = ((self.registers[source_reg_1] as i16) + (data as i16)) as u16;
        Ok(())
    }

    // Instruction format [OP_CODE(4 bit), Dest Register (3 bit), 0, Value-Sign (1-bit), Value-Number (7 bit)];
    fn load(&mut self, instr: u16) -> anyhow::Result<()> {
        let reg = ((instr >> 9) & 7) as usize;
        self.registers[reg] = get_sign_extended_value(instr & ((1 << 8) - 1), 8);
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

fn get_sign_extended_value(mut val: u16, msb_index: usize) -> u16 {
    // If value's sign bit is 1, value is negative.
    if (val >> (msb_index - 1)) % 2 == 1 {
        // Create a mask which sets all bits left to the msb of the value to 1 to represent it as negative in 16 bit.
        let mut mask: u16 = 0;
        let mut bit = 16;
        loop {
            mask |= if bit >= msb_index - 1 { 1 } else { 0 };
            mask <<= 1;
            if bit == 0 {
                break;
            }
            bit -= 1;
        }
        val |= mask
    }
    val
}

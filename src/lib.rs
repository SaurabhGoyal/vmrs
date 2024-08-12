use std::io::{stdin, Read};

const MEMORY_SLOT_COUNT: usize = 1 << 4;

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
const RSTAT_CONDITION_ZERO: u16 = 0;
const RSTAT_CONDITION_POSITIVE: u16 = 1;
const RSTAT_CONDITION_NEGATIVE: u16 = 2;
const RSTAT_CONDITION_HALT: u16 = 3;

const OP_BREAK: u16 = 0;
// Instruction formats-
// - Register mode - [OP_CODE(4 bits), Dest Register (3 bits), Source-Register-1 (3 bits), 000, Source-Register-2 (3 bits)]
// - Immediate mode - [OP_CODE(4 bits), Dest Register (3 bits), Source-Register-1 (3 bits), 1, Value-Sign (1-bit), Value-Number (4 bits)]
const OP_ADD: u16 = 1;
// Instruction format [OP_CODE(4 bits), Dest Register (3 bits), 0, Value-Sign (1-bits), Value-Number (7 bits)]
const OP_LOAD: u16 = 2;
// Instruction format [OP_CODE(4 bits), Dest Register (3 bits), Relative-Memory-Address (9 bits)];
const OP_LOAD_INDIRECT: u16 = 3;
// Instruction format [OP_CODE(4 bits), 000 (3 bits), Relative-Memory-Address (9 bits)];
const OP_JUMP: u16 = 4;
// Instruction format [OP_CODE(4 bits), Register (3 bits), Relative-Memory-Address (9 bits)];
const OP_JUMP_IF_SIGN: u16 = 5;
// Instruction format [OP_CODE(4 bits), Dest Register (3 bits), Source-Register-1 (3 bits), 000000 (6 bits)];
const OP_LOAD_REGISTER: u16 = 6;
// Instruction format [OP_CODE(4 bits), 0000 (4 bits), TrapCode (12 bits)]
const OP_TRAP: u16 = 15;

// Halt the program
const TRAP_CODE_HALT: u16 = 0;
// Get character from keyboard
const TRAP_CODE_GETC: u16 = 1;

const REG_COUNT: usize = 10;
const MAX_LOOP_ITERS: u16 = 30;
const LSB_MASK_12_BIT: u16 = 4095; // 0000111111111111

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
        self.slots[addr..(addr + values.len())].copy_from_slice(values);
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
    pub fn load(&mut self, addr: u16, data: &[u16]) -> anyhow::Result<()> {
        self.memory.write(addr as usize, data)?;
        Ok(())
    }
    pub fn run(&mut self, addr: u16) -> anyhow::Result<()> {
        self.registers[RPC] = addr;
        let mut ic = 0;
        loop {
            // Execute step
            self.step()?;
            println!("Step - {:?}", self.dump().registers);
            ic += 1;
            // Check if need to halt
            if self.registers[RSTAT] == RSTAT_CONDITION_HALT || ic == MAX_LOOP_ITERS {
                break;
            }
        }
        Ok(())
    }

    // Instruction format [OP_CODE(4 bits), Parameters (12 bits)];
    fn step(&mut self) -> anyhow::Result<()> {
        let pc = self.registers[RPC];
        let instr = self.memory.read(pc as usize, 1)?[0];
        let op_code = instr >> 12;
        match op_code {
            OP_BREAK => self.registers[RSTAT] = RSTAT_CONDITION_HALT,
            OP_ADD => self.instr_add(instr)?,
            OP_LOAD => self.instr_load(instr)?,
            OP_LOAD_INDIRECT => self.instr_load_indirect(instr)?,
            OP_LOAD_REGISTER => self.instr_load_register(instr)?,
            OP_JUMP => self.instr_jump(instr)?,
            OP_JUMP_IF_SIGN => self.instr_jump_if_sign(instr)?,
            OP_TRAP => self.instr_trap(instr)?,
            _ => {}
        }
        Ok(())
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

    fn instr_add(&mut self, instr: u16) -> anyhow::Result<()> {
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
        self.write_to_register(
            dest_reg,
            ((self.registers[source_reg_1] as i16) + (data as i16)) as u16,
        )?;
        self.registers[RPC] += 1;
        Ok(())
    }

    fn instr_load(&mut self, instr: u16) -> anyhow::Result<()> {
        let reg = ((instr >> 9) & 7) as usize;
        self.write_to_register(reg, get_sign_extended_value(instr & ((1 << 8) - 1), 8))?;
        self.registers[RPC] += 1;
        Ok(())
    }

    fn instr_load_indirect(&mut self, instr: u16) -> anyhow::Result<()> {
        let reg = ((instr >> 9) & 7) as usize;
        let relative_addr = get_sign_extended_value(instr & ((1 << 9) - 1), 9) as i16;
        let abs_addr = (self.registers[RPC] as i16 + relative_addr) as u16;
        self.write_to_register(reg, self.memory.read(abs_addr as usize, 1)?[0])?;
        self.registers[RPC] += 1;
        Ok(())
    }

    fn instr_load_register(&mut self, instr: u16) -> anyhow::Result<()> {
        let dest_reg = ((instr >> 9) & 7) as usize;
        let source_reg = ((instr >> 6) & 7) as usize;
        self.write_to_register(dest_reg, self.registers[source_reg])?;
        self.registers[RPC] += 1;
        Ok(())
    }

    fn instr_jump(&mut self, instr: u16) -> anyhow::Result<()> {
        let relative_addr = get_sign_extended_value(instr & ((1 << 9) - 1), 9) as i16;
        let abs_addr = (self.registers[RPC] as i16 + relative_addr) as u16;
        self.registers[RPC] = abs_addr;
        Ok(())
    }

    fn instr_jump_if_sign(&mut self, instr: u16) -> anyhow::Result<()> {
        let reg = ((instr >> 9) & 7) as usize;
        let relative_addr = get_sign_extended_value(instr & ((1 << 9) - 1), 9) as i16;
        let abs_addr = (self.registers[RPC] as i16 + relative_addr) as u16;
        if self.registers[RSTAT] == RSTAT_CONDITION_NEGATIVE {
            self.registers[RPC] = abs_addr;
        } else {
            self.registers[RPC] += 1;
        }
        Ok(())
    }

    fn instr_trap(&mut self, instr: u16) -> anyhow::Result<()> {
        let trap_code = instr & ((1 << 8) - 1);
        match trap_code {
            TRAP_CODE_HALT => self.trap_halt()?,
            TRAP_CODE_GETC => self.trap_getc()?,
            _ => {
                self.registers[RPC] += 1;
            }
        }
        Ok(())
    }

    fn trap_halt(&mut self) -> anyhow::Result<()> {
        self.registers[RSTAT] = RSTAT_CONDITION_HALT;
        Ok(())
    }

    fn trap_getc(&mut self) -> anyhow::Result<()> {
        if let Some(Ok(byte)) = stdin().bytes().next() {
            self.registers[R0] = byte as u16;
        }
        self.registers[RPC] += 1;
        Ok(())
    }

    fn write_to_register(&mut self, dest_reg: usize, val: u16) -> anyhow::Result<()> {
        self.registers[dest_reg] = val;
        self.registers[RSTAT] = match val {
            0 => RSTAT_CONDITION_ZERO,
            _ => match val >> 15 {
                0 => RSTAT_CONDITION_POSITIVE,
                _ => RSTAT_CONDITION_NEGATIVE,
            },
        };
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

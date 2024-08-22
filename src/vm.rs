const MEMORY_SLOT_COUNT: usize = 1 << 4;

// `R0`-`R7` are data storage registers. They are modelled as unsigned int16 but machine itself doesn't
// assign any numeric schemantic to them. For machine, they are 16-bit data storage which may store
// anything in it.
const _R0: usize = 0;
const _R1: usize = 1;
const _R2: usize = 2;
const _R3: usize = 3;
const _R4: usize = 4;
const _R5: usize = 5;
const _R6: usize = 6;
const _R7: usize = 7;
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
pub const RSTAT_HALT: u16 = 3;
pub const RSTAT_WAITING_FOR_INPUT: u16 = 4;

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
// Instruction format [OP_CODE(4 bits), Register (3 bits), Relative-Memory-Address (9 bits)];
const OP_JUMP_IF_ZERO: u16 = 7;
// Instruction format [OP_CODE(4 bits), Register (3 bits), Relative-Memory-Address (9 bits)];
const OP_JUMP_IF_NO_SIGN: u16 = 8;
// Instruction format [OP_CODE(4 bits), 0000 (4 bits), TrapCode (12 bits)]
const OP_TRAP: u16 = 15;

// Halt the program
const TRAP_CODE_HALT: u16 = 0;
// Get character from keyboard
const TRAP_CODE_GETC: u16 = 1;

// Memory Segmentation allows controlling access to the memory for different purposes. A memory address can be written to only if
// its segment is either unintialised or is the same as the desired segment of the data which is being written. Ex.- Once machine has
// initialised some memory segment as INT_HANDLER_TABLE, it can be written only if the table is being updated and not if some program
// data has to be written on it.
const SEGMENT_UNINT: u8 = 0;
// This is for storing data from io devices
const _SEGMENT_INT_DATA: u8 = 1;
// This is for storing interrupt handler table
const SEGMENT_INT_HANDLER_TABLE: u8 = 2;
// This is for storing interrupt handler program code
const _SEGMENT_INT_PROGRAM_CODE: u8 = 3;
// This is for storing static program data
const _SEGMENT_PROGRAM_DATA: u8 = 4;
// This is for storing program data
const _SEGMENT_PROGRAM_CODE: u8 = 5;
// This is for storing dynamic data
const _SEGMENT_DYNAMIC_DATA: u8 = 6;

const REG_COUNT: usize = 10;

type MemorySlot = (u8, u16);

trait Addressable {
    fn read(&self, addr: usize, slot_count: usize) -> anyhow::Result<&[MemorySlot]>;
    fn write(&mut self, addr: usize, values: &[MemorySlot]) -> anyhow::Result<()>;
    fn dump(&self) -> &[MemorySlot];
}

#[derive(Debug)]
struct Memory {
    slots: [MemorySlot; MEMORY_SLOT_COUNT],
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            slots: [(SEGMENT_UNINT, 0); MEMORY_SLOT_COUNT],
        }
    }
}

impl Addressable for Memory {
    fn read(&self, addr: usize, slot_count: usize) -> anyhow::Result<&[MemorySlot]> {
        if addr + slot_count > self.slots.len() {
            anyhow::bail!("Invalid address and slot count");
        }
        Ok(self.slots[addr..addr + slot_count].as_ref())
    }

    fn write(&mut self, addr: usize, values: &[MemorySlot]) -> anyhow::Result<()> {
        if addr + values.len() > self.slots.len() {
            anyhow::bail!("Invalid value size for given address");
        }
        for (index, (segment, data)) in values.iter().enumerate() {
            let curr_segment = self.slots[addr + index].0;
            if curr_segment != SEGMENT_UNINT && curr_segment != *segment {
                anyhow::bail!("Slot can not be overridden with new segment once assigned");
            }
            self.slots[addr + index] = (*segment, *data);
        }
        Ok(())
    }

    fn dump(&self) -> &[MemorySlot] {
        self.slots.as_slice()
    }
}

pub struct Machine {
    registers: [u16; REG_COUNT],
    memory: Box<dyn Addressable>,
    interrupt_table_address: Option<u16>,
}

#[derive(Debug)]
pub struct Dump {
    pub registers: [i16; REG_COUNT],
    pub memory: Vec<(u8, i16)>,
}

impl Machine {
    pub fn load(&mut self, segment: u8, addr: u16, data: &[u16]) -> anyhow::Result<()> {
        self.memory.write(
            addr as usize,
            data.iter()
                .map(|v| (segment, *v))
                .collect::<Vec<MemorySlot>>()
                .as_slice(),
        )?;
        if segment == SEGMENT_INT_HANDLER_TABLE {
            self.interrupt_table_address = Some(addr);
        }
        Ok(())
    }
    pub fn set_pc(&mut self, addr: u16) -> anyhow::Result<()> {
        self.registers[RPC] = addr;
        self.registers[RSTAT] = RSTAT_CONDITION_ZERO;
        Ok(())
    }

    // Instruction format [OP_CODE(4 bits), Parameters (12 bits)];
    pub fn execute_instruction(&mut self) -> anyhow::Result<u16> {
        if !matches!(self.registers[RSTAT], RSTAT_HALT | RSTAT_WAITING_FOR_INPUT) {
            let pc = self.registers[RPC];
            let (_, instr) = self.memory.read(pc as usize, 1)?[0];
            let op_code = instr >> 12;
            match op_code {
                OP_BREAK => self.registers[RSTAT] = RSTAT_HALT,
                OP_ADD => self.instr_add(instr)?,
                OP_LOAD => self.instr_load(instr)?,
                OP_LOAD_INDIRECT => self.instr_load_indirect(instr)?,
                OP_LOAD_REGISTER => self.instr_load_register(instr)?,
                OP_JUMP => self.instr_jump(instr)?,
                OP_JUMP_IF_SIGN => self.instr_jump_if_sign(instr)?,
                OP_JUMP_IF_NO_SIGN => self.instr_jump_if_no_sign(instr)?,
                OP_JUMP_IF_ZERO => self.instr_jump_if_zero(instr)?,
                OP_TRAP => self.instr_trap(instr)?,
                _ => {}
            }
        }
        Ok(self.registers[RSTAT])
    }

    // Assumes multiple things
    // - Interrupt to handler address table must have been loaded in VM.
    // - Interrupt handler code must not modify any register other than R0.
    // - Interrupt handler code must have a HALT instruction in the end.
    pub fn handle_interrupt(&mut self, _device_id: u8, interrupt: u16) -> anyhow::Result<()> {
        let mut interrupt_address = *self.interrupt_table_address.as_ref().unwrap() as usize;
        loop {
            let (segment, value) = self.memory.read(interrupt_address, 1).unwrap()[0];
            if segment != SEGMENT_INT_HANDLER_TABLE {
                break;
            }
            if value == interrupt {
                interrupt_address += 1;
                let (segment, value) = self.memory.read(interrupt_address, 1).unwrap()[0];
                if segment != SEGMENT_INT_HANDLER_TABLE {
                    anyhow::bail!("Interrupt table segment not found at the interrupt address");
                }
                let curr_pc = self.registers[RPC];
                self.registers[RPC] = value;
                self.registers[RSTAT] = RSTAT_CONDITION_ZERO;
                while self.registers[RSTAT] != RSTAT_HALT {
                    self.execute_instruction().unwrap();
                }
                self.registers[RPC] = curr_pc + 1;
                self.registers[RSTAT] = RSTAT_CONDITION_ZERO;
                println!("Interrupt handled by executing program at address {value}");
                break;
            }
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
                .map(|(s, n)| (*s, *n as i16))
                .collect::<Vec<(u8, i16)>>(),
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
        self.write_to_register(reg, self.memory.read(abs_addr as usize, 1)?[0].1)?;
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
        if self.registers[RSTAT] == RSTAT_CONDITION_NEGATIVE {
            let relative_addr = get_sign_extended_value(instr & ((1 << 9) - 1), 9) as i16;
            let abs_addr = (self.registers[RPC] as i16 + relative_addr) as u16;
            self.registers[RPC] = abs_addr;
        } else {
            self.registers[RPC] += 1;
        }
        Ok(())
    }

    fn instr_jump_if_no_sign(&mut self, instr: u16) -> anyhow::Result<()> {
        if self.registers[RSTAT] == RSTAT_CONDITION_POSITIVE {
            let relative_addr = get_sign_extended_value(instr & ((1 << 9) - 1), 9) as i16;
            let abs_addr = (self.registers[RPC] as i16 + relative_addr) as u16;
            self.registers[RPC] = abs_addr;
        } else {
            self.registers[RPC] += 1;
        }
        Ok(())
    }

    fn instr_jump_if_zero(&mut self, instr: u16) -> anyhow::Result<()> {
        if self.registers[RSTAT] == RSTAT_CONDITION_ZERO {
            let relative_addr = get_sign_extended_value(instr & ((1 << 9) - 1), 9) as i16;
            let abs_addr = (self.registers[RPC] as i16 + relative_addr) as u16;
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
        self.registers[RSTAT] = RSTAT_HALT;
        Ok(())
    }

    fn trap_getc(&mut self) -> anyhow::Result<()> {
        self.registers[RSTAT] = RSTAT_WAITING_FOR_INPUT;
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
            interrupt_table_address: None,
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

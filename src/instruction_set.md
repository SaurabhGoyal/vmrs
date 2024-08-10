# Instruction-Set
## General syntax
`OP_CODE - (4 bits) | OPERANDS (12 bits)` 

# Commands
## Data (OpCode - 1110)
- Stores given value in memory.
- Syntax - `1110 (4 bits) | Data (12 bits)`

## ADD (OpCode - 0001)
- Adds two values and puts the result into a destination register. The values van be either fetched from registers or given in-place.
- Syntax - `0001 (4 bits) | Dest Register (3 bits) | Source-Register-1 (3 bits) | Mode (1 bit) | Mode-Specific-Operands (5 bits)`
- Two modes
    - Register mode (Mode bit 0) - When second operand data is in another source register.
    - Syntax - `0001 (4 bits) | Dest Register (3 bits) | Source-Register-1 (3 bits) | 0 (1 bit) | 00 (2 bits) | Source-Register-2 (3 bits)`
    - Immediate mode (Mode bit 1) - When second operand data is in the instruction itself.
    - Syntax - `0001 (4 bits) | Dest Register (3 bits) | Source-Register-1 (3 bits) | 1 (1 bit) | Sign-of-Value (1 bit) | Number-of-Value (4 bits)`

## Load (OpCode - 0010)
- Loads the value given in the instruction to the destination register for later use.
- Syntax - `0010 (4 bits) | Dest Register (3 bits) | 0 (1 bit) | Sign-of-Value (1 bit) | Number-of-Value (7 bits)`

## LoadIndirect (OpCode - 0011)
- Loads the value at the memory address given in the instruction to the destination register for later use.
- Important thing to note is that the address will be relative to the program code instructions and not absolute. This means that the relative address can be negative and should be sign-extended. 
- Syntax - `0011 (4 bits) | Dest Register (3 bits) | Relative-Memory-Address (9 bits)`

## LoadRegister (OpCode - 0110)
- Loads the value stored in the source register given in the instruction to the destination register for later use.
- Syntax - `0001 (4 bits) | Dest Register (3 bits) | Source-Register (3 bits) | 000000 (6 bits)`

## Jump (OpCode - 0100)
- Sets program counter to the memory address given in the instruction.
- Enables non-liner execution of program.
- Syntax - `0100 (4 bits) | 000 (3 bits) | Relative-Memory-Address (9 bits)`

## JumpIfSign (OpCode - 0101)
- Sets program counter to the memory address given in the instruction if the register given in instruction has negative value.
- Enables non-liner execution of program.
- Syntax - `0101 (4 bits) | Dest Register (3 bits) | Relative-Memory-Address (9 bits)`

## Trap (OpCode - 1111)
- Sets a trap to the instruction execution for machine to do things outside the instruction in the program code.
- These things can be things such as talking to IO decices or halting the program. Simple way to imagine is that this is a set of machine defined functionalities to interact with outside the program-code and machine scope.
- Since these things are something that machine implements on its own and are not part of user-defined instructions, machine implements them itself and stores the implementation logic in memory. This is the reason, machine may need some part of its memory for its own things and program code would be stored at a non-zero address, typically 0x3000.
- Because of multiple types of traps and their distinct nature of being a foreign-function-interface instruction, they have been categorised into one opcode where the trap type can be passed as an operand, instead of creating a dedicated opcode for each trap. https://www.jmeiners.com/lc3-vm/#trap-routines
    > Trap for additional functionalities You may be wondering why the trap codes are not included in the instructions. This is because they do not actually introduce any new functionality to the LC-3, they just provide a convenient way to perform a task (similar to OS system calls). In the official LC-3 simulator, trap routines are written in assembly. When a trap code is called, the PC is moved to that code’s address. The CPU executes the procedure’s instructions, and when it is complete, the PC is reset to the location following the initial call.

    > Note: This is why programs start at address 0x3000 instead of 0x0. The lower addresses are left empty to leave space for the trap routine code.
- Syntax - `1111 (4 bits) | 0000 (4 bits) | TrapCode (8 bits)`

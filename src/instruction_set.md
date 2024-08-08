# Instruction-Set
## General syntax
`OP_CODE - (4 bits) | OPERANDS (12 bits)` 

# Commands
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

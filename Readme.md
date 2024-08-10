# Overview
Implementation of 16-bit virtual machine (VM).

# Run
## Add positive numbers
```
RUST_BACKTRACE=1 cargo run -q < src/sample_programs/adder_pos_nums.o

Final - [3, 6, 9, 17, 0, 0, 0, 0, 4, 0]
```

## Add negative numbers with total sum as positive
```
RUST_BACKTRACE=1 cargo run -q < src/sample_programs/adder_neg_nums_pos_result.o

Final - [3, 6, 9, 2, 0, 0, 0, 0, 4, 0]
```

## Add negative numbers with total sum as negative
```
RUST_BACKTRACE=1 cargo run -q < src/sample_programs/adder_neg_nums_neg_result.o

Final - [3, 6, 9, -9, 0, 0, 0, 0, 4, 0]
```

## Add negative numbers with total sum as negative using load indirect op
```
RUST_BACKTRACE=1 cargo run -q < src/sample_programs/adder_neg_nums_neg_result_indirect_load.o

Final - [3, 6, 9, -9, 0, 0, 0, 0, 4, 0]
```
## Infinite loop (With forced cutoff in run function after 30 iterations)
```
RUST_BACKTRACE=1 cargo run -q < src/sample_programs/loop_infinite.o

Step - [3, 0, 0, 0, 0, 0, 0, 0, 0, 1]
Step - [3, 6, 0, 0, 0, 0, 0, 0, 1, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
```

## Finite loop
```
RUST_BACKTRACE=1 cargo run -q < src/sample_programs/loop_finite.o

Step - [2, 0, 0, 0, 0, 0, 0, 0, 0, 1]
Step - [2, -5, 0, 0, 0, 0, 0, 0, 1, 2]
Step - [2, -5, -3, 0, 0, 0, 0, 0, 2, 2]
Step - [2, -5, -3, 1, 0, 0, 0, 0, 3, 1]
Step - [2, -5, -2, 1, 0, 0, 0, 0, 4, 2]
Step - [2, -5, -2, 1, 0, 0, 0, 0, 5, 2]
Step - [2, -5, -1, 1, 0, 0, 0, 0, 4, 2]
Step - [2, -5, -1, 1, 0, 0, 0, 0, 5, 2]
Step - [2, -5, 0, 1, 0, 0, 0, 0, 4, 0]
Step - [2, -5, 0, 1, 0, 0, 0, 0, 5, 0]
```

## Finite loop
```
RUST_BACKTRACE=1 cargo run -q < src/sample_programs/fib.o

Step - [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
Step - [0, 1, 0, 0, 0, 0, 0, 0, 1, 1]
Step - [0, 1, 0, 0, 0, 0, 0, 3, 2, 1]
Step - [0, 1, 1, 0, 0, 0, 0, 3, 3, 1]
Step - [0, 1, 1, 0, 0, 0, 0, 3, 4, 1]
Step - [1, 1, 1, 0, 0, 0, 0, 3, 5, 1]
Step - [1, 1, 1, 0, 0, 0, 0, -1, 6, 2]
Step - [1, 1, 1, 0, 0, 0, 0, -1, 7, 2]
Step - [1, 1, 1, 0, 0, 0, 0, -1, 3, 1]
Step - [1, 2, 1, 0, 0, 0, 0, -1, 4, 1]
Step - [1, 2, 1, 0, 0, 0, 0, -1, 5, 1]
Step - [1, 2, 1, 0, 0, 0, 0, -1, 6, 2]
Step - [1, 2, 1, 0, 0, 0, 0, -1, 7, 2]
Step - [1, 2, 2, 0, 0, 0, 0, -1, 3, 1]
Step - [1, 3, 2, 0, 0, 0, 0, -1, 4, 1]
Step - [2, 3, 2, 0, 0, 0, 0, -1, 5, 1]
Step - [2, 3, 2, 0, 0, 0, 0, -1, 6, 2]
Step - [2, 3, 2, 0, 0, 0, 0, -1, 7, 2]
Step - [2, 3, 3, 0, 0, 0, 0, -1, 3, 1]
Step - [2, 5, 3, 0, 0, 0, 0, -1, 4, 1]
Step - [3, 5, 3, 0, 0, 0, 0, -1, 5, 1]
Step - [3, 5, 3, 0, 0, 0, 0, -1, 6, 2]
Step - [3, 5, 3, 0, 0, 0, 0, -1, 7, 2]
Step - [3, 5, 5, 0, 0, 0, 0, -1, 3, 1]
Step - [3, 8, 5, 0, 0, 0, 0, -1, 4, 1]
Step - [5, 8, 5, 0, 0, 0, 0, -1, 5, 1]
Step - [5, 8, 5, 0, 0, 0, 0, -1, 6, 2]
Step - [5, 8, 5, 0, 0, 0, 0, -1, 7, 2]
Step - [5, 8, 8, 0, 0, 0, 0, -1, 3, 1]
Step - [5, 13, 8, 0, 0, 0, 0, -1, 4, 1]
Final - [5, 13, 8, 0, 0, 0, 0, -1, 5, 1]
```

# Implementation
## What value and API does a VM provide?
- A virtual machine is a mock for a real machine, in our case a CPU.
- The main value that CPU provides is to run instructions.
- We would follow the LC-3 architecture of a CPU to build our virtual machine.
- To run the machine, a program code has to be provided which is nothing but an array of instructions.
- The instructions themselves need to be in machine code, each instruction being a 16-bit value.
- The machine will simply execute and step through each instruction of the program one at a time.
- The instruction-set would be of Assembly language as it is smaller and easier to implement.

## How does it work internally?
- Machine has two components -
    - Registers - For control purpose. Each register can hold a 16 bit value. There are 10 registers in this implementation -
        - R0-R7 - value storage during instruction execution
        - RPC - program counter for machine to track which instruction to be executed
        - RCOND - value storage of previous instructions for conditional instructions
    - Memory - For data storage purpose only. This provides a larger storage area than registers and is used purely for storage of data that can not be fit into storage registers (R0-R7). Each memory slot can also hold a 16-bit value.
- Machine loads the program code into memory at a certain address and sets `RPC` to that.
- Machine keeps loading the instruction at the memory address referred by `RPC` and executing the business logic of that instruction. If the instruction is `OP_TRAP`, machine exits the program.

## Doubts
- **Why to model registers as unsigned ints and then handle the negative numbers manually in VM logic instead of modelling them as signed ints only?**
    - Because registers in hardware are simple bit storage devices and do NOT care about the data they hold, i.e. they do not have a direct understanding of numbers, let alone positive or negative. This also keeps the hardware API simpler for different users to build whatever logic they want to build on top of a bit array register.
    - Ref - https://stackoverflow.com/a/27207704/2555504
- **Why do we need Program-Counter reigtser (`RPC`)?**
    - To support non-linear execution of code which is powered by `go-to / jump` statement enabling connstructs such as `if-else` and `loop`.
- **Why do we need a dedicated register (`RSTAT`) for maintaining the sign of the result of previous instruction when the same can be checked from the result itself?**
    - `RSTAT` is a dedicated register for a quick lookup of multiple things such as sign of last result (+ve / -ve), status of last operation (underflow / overflow), augmented information of last result (carry) and various interrupts. While the sign can be directly checked from result, the check is mostly conducted in some kind of branching decision context which is where status register provides information in generic sense.
    - This register has been named as `RCOND` in the referring blog post and is also called as `Condition Code Register` or simply `Condition Register` sometimes.
- **We are using a dedicated op-code for not treating an instruction as operation, for storing raw data in memory. This wastes 4 bits, is there any workaround?**
    - [Pending]
- **Why is an address needed to load the program code? While I haven't used any custom address, i.e. loaded the program code simply at 0th address, the blog post suggest to use 0x3000, why?**
    - The reason is simply that in real world, machine may have more things that it needs to manage in the memory other than just the program code to be executed. One such thing is trap routine code which is nothing but some special instructions that machine itself has hardcoded to provide functionalities such as talking to IO devices and halt the program. 

# References
- https://www.jmeiners.com/lc3-vm/#:lc3.c_2
- https://www.youtube.com/watch?v=oArXOAhzOdY&list=PLUkZG7_4JtUL22HycWYR_J-1xJo7rQGhr
- https://www.andreinc.net/2021/12/01/writing-a-simple-vm-in-less-than-125-lines-of-c


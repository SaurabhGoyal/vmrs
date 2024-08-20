# Overview
Implementation of 16-bit virtual machine (VM). **Please note that this implementation is inspired by LC3 but does not completely follow it, infact implements instructions and design using my own intuition.**

# Implementation
## What value and API does a VM provide?
- A virtual machine is a mock for a real machine, in our case a CPU.
- The main value that CPU provides is to run instructions.
- This implementation is inspired by `LC-3 architecture` of a CPU to build the virtual machine BUT would not implement or follow it completely. 
- To run the machine, a program code has to be provided which is nothing but an array of instructions.
- The instructions themselves need to be in machine code, each instruction being a 16-bit value.
- The machine will simply execute and step through each instruction of the program one at a time.
- The instruction-set would be of Assembly language as it is smaller and easier to implement.
- The above is sufficient if we do have a machine which can not be interrupted (given additional input) during its execution.
- To add support of `interruption` of machine, we need a way for machine to be able to take new input, process it and generate a response of the execution of the input.     

## How does it work internally?
- Machine needs to do take data, take instructions and do their calculations and produce results.
- Machine also needs to listen to external events such as IO devices.
- Machine needs a way for taking and storing the data, instructions and results. This is where **memory** comes.
- Machine needs to perform calculations for instructions, that calculation happens in the logic gates implemented in the hardware of the machine and is encapsulated in the form of **registers**.
- Data need to be sent to registers from memory and register is instructed to perform the operation.
- [`Interrupts`](https://en.wikipedia.org/wiki/Interrupt) can be given only between instructions and not in between. In real hardware interrupts are enabled by having dedicated interrupt input pins for external systems which set the value as per the interrupt they want to trigger and the circuit of CPU is built in a way that in each instruction cycle, if the pin values are off, the normal instruction would be executed and if not, the circuit of interrupt handling branch will get enabled. In software, dedicated registers (special purpose registers) or dedicated memory (memory mapped registers) can be used to track the interrupt status.
- In this VM however, we would make the yield control back after each instruction cycle so that pending interrupts can be read and processed.
- An additional module, called, `Interrupt Controller` which can take input from user / external systems and pass on to CPU is added as well. A separate module is needed because we want the machine to not be impacted mid-instruction execution cycle if interrupts are generated and we don't want external modules to not be able to create interrupts if the CPU is busy.

## Components
Three components to enable above -
- Machine - simulates the CPU
- Interrupt Controller - simulates the interrupt pins and dedicated interrupt controller in a real hardware
- Simulator (`main.rs`) - performs two tasks -
    - Enables an user-interface (CLI) for external systems (programmer) to use the machine as well interrupt controller.
    - Runs the above two systems in dedicated threads for them to be non blocking. 

### Machine
- Machine has three components -
    - **Registers -** For control purpose. Each register can hold a 16 bit value. There are 10 registers in this implementation -
        - R0-R7 - value storage during instruction execution
        - RPC - program counter for machine to track which instruction to be executed
        - RCOND - value storage of previous instructions for conditional instructions
    - **Memory -** For data storage purpose only. This provides a larger storage area than registers and is used purely for storage of data that can not be fit into storage registers (R0-R7). Each memory slot can also hold a 16-bit value.
        - Machine also provides a virtual component called **"Memory Mapped Registers"** which is nothing but memory area which will be used for control purpose of dynamic usecases such as handling IO from devices. Dedicated registers are not used for this because memory is more dispensable than registers.
- Machine exposes three APIs -
    - `Load()` - any arbitrary data in the memory. Programmer should use it to vm load data and program code into memory at desired addresses. (This is programmer's responsibility that the addresses of the data in the program code point correctly to the loaded data in memory.)
    - `Run()` - run the program code stored at given address. Machine sets `RPC` to that address and performs exactly one instruction cycle.
    - `HandleInterrupt(interrupt)` - runs the handler code corresponding to the given interrupt. This will be called manually by interrupt controller if needed. The handling happens in below steps -
        - For each interrupt id that needs to be handled, the logic (instructions) to handle it should be loaded into the memory using `load` API.
        - A mapping of interrupt id to memory address of interrupt handler logic should be loaded into the memory using `load` API.
        - CPU also needs to know the address of this mapping in memory to refer to. I am not sure how real machine does it, for now, I am going to use a hardcoded address of `0x0` to store the mapping. This would work because size of each part of the table and program code can be pre-calculated definteively by the programmer before uploading them into the machine.
- Machine keeps loading the instruction at the memory address referred by `RPC` and executing the business logic of that instruction. If the instruction is `OP_TRAP` with `TRAP_HALT`, machine exits the program.

### Interrupt Controller
- Interrupt Controller has a single component **Pending Interrupt Queue** where any incoming and non-handled interrupts are stored.
- We will model each interrupt as a single 8 bit data item with following format - `Device-id (4 bits) | Interrupt-id (4 bits)`. Interrupts themselves are simple signal mechanisms of an event happening and do not carry with them the data of the event. The data of the event should be put into an area commonly accessible by the external system as well as CPU. Such area can be CPU memory and special registers (such as Memory mapped registers) or dedicated managed-memory (separate from CPU, accessible via MMU - Memory Management Unit). In the former case, external system can use vm's `load` API and then use interrupt-controller's `int` API.
- It exposes two APIs -
    - `Int(interrupt)` - Adds an interrupt to the pending queue. It may discard them as well depending on if they are duplicate or too soon or any other factor. This will be called by the external systems. 
    - `IntA(interrupt)` - Acknowledges an interrupt handling to be complete and removes it from pending interrupts queue. This will be called based on interrupt handling response from CPU. 

## Doubts
- **Why to model registers as unsigned ints and then handle the negative numbers manually in VM logic instead of modelling them as signed ints only?**
    - Because registers in hardware are simple bit storage devices and do NOT care about the data they hold, i.e. they do not have a direct understanding of numbers, let alone positive or negative. This also keeps the hardware API simpler for different users to build whatever logic they want to build on top of a bit array register.
    - Ref - https://stackoverflow.com/a/27207704/2555504
- **Why do we need Program-Counter reigtser (`RPC`)?**
    - To support non-linear execution of code which is powered by `go-to / jump` statement enabling connstructs such as `if-else` and `loop`.
- **Why do we need a dedicated register (`RSTAT`) for maintaining the sign of the result of previous instruction when the same can be checked from the result itself?**
    - `RSTAT` is a dedicated register for a quick lookup of multiple things such as sign of last result (+ve / -ve), status of last operation (underflow / overflow), augmented information of last result (carry) and various interrupts. While the sign can be directly checked from result, the check is mostly conducted in some kind of branching decision context which is where status register provides information in generic sense.
    - This register has been named as `RCOND` in the referring blog post and is also called as `Condition Code Register` or simply `Condition Register` sometimes.
- ~~**We are using a dedicated op-code for not treating an instruction as operation, for storing raw data in memory. This wastes 4 bits, is there any workaround?**~~
    - Using the two step (load, run) process now instead of the single step (run-with-load), resolving this issue.
- **Why is an address needed to vm load the program code? While I haven't used any custom address, i.e. loaded the program code simply at 0th address, the blog post suggest to use 0x3000, why?**
    - The reason is simply that in real world, machine may have more things that it needs to manage in the memory other than just the program code to be executed. One such thing is trap routine code which is nothing but some special instructions that machine itself has hardcoded to provide functionalities such as talking to IO devices and halt the program. 


# Run
`RUST_BACKTRACE=1 cargo run -q` 
### Add positive numbers
```
vm load src/sample_programs/adder_pos_nums.o 0
vm run 0
vm load - Dump { registers: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], memory: [8195, 8710, 5121, 5800, -4096, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
Step - [3, 0, 0, 0, 0, 0, 0, 0, 1, 1]
Step - [3, 6, 0, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 17, 0, 0, 0, 0, 4, 1]
Step - [3, 6, 9, 17, 0, 0, 0, 0, 4, 3]
Final - Dump { registers: [3, 6, 9, 17, 0, 0, 0, 0, 4, 3], memory: [8195, 8710, 5121, 5800, -4096, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
```

### Add negative numbers with total sum as positive
```
vm load src/sample_programs/adder_neg_nums_pos_result.o 0
vm run 0
vm load - Dump { registers: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], memory: [8195, 8710, 5121, 5817, -4096, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
Step - [3, 0, 0, 0, 0, 0, 0, 0, 1, 1]
Step - [3, 6, 0, 0, 0, 0, 0, 0, 2, 1]
Step - [3, 6, 9, 0, 0, 0, 0, 0, 3, 1]
Step - [3, 6, 9, 2, 0, 0, 0, 0, 4, 1]
Step - [3, 6, 9, 2, 0, 0, 0, 0, 4, 3]
Final - Dump { registers: [3, 6, 9, 2, 0, 0, 0, 0, 4, 3], memory: [8195, 8710, 5121, 5817, -4096, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
```

### Add negative numbers with total sum as negative
```
vm load src/sample_programs/adder_neg_nums_neg_result.o 0
vm run 0
Final - Dump { registers: [3, 6, 9, 2, 0, 0, 0, 0, 4, 3], memory: [8195, 8710, 5121, 5817, -4096, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
```

### Add negative numbers with total sum as negative using vm load indirect op
```
vm load src/sample_programs/data_load_negative_num.o 0
vm load src/sample_programs/adder_neg_nums_neg_result_indirect_load.o 1
vm run 1
vm load - Dump { registers: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], memory: [-5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
vm load - Dump { registers: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], memory: [-5, 8194, 13310, 5121, 5818, -4096, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
Step - [2, 0, 0, 0, 0, 0, 0, 0, 2, 1]
Step - [2, -5, 0, 0, 0, 0, 0, 0, 3, 2]
Step - [2, -5, -3, 0, 0, 0, 0, 0, 4, 2]
Step - [2, -5, -3, -9, 0, 0, 0, 0, 5, 2]
Step - [2, -5, -3, -9, 0, 0, 0, 0, 5, 3]
Final - Dump { registers: [2, -5, -3, -9, 0, 0, 0, 0, 5, 3], memory: [-5, 8194, 13310, 5121, 5818, -4096, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
```
### Infinite loop (With forced cutoff in vm run function after 30 iterations)
```
vm load src/sample_programs/loop_infinite.o 0
vm run 0

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

### Finite loop
```
vm load src/sample_programs/loop_finite.o 0
vm run 0

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

### Fibonacci series till nth number
```
vm load src/sample_programs/fib.o 0
vm run 0

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

### Trap with getchar and halt
```
vm load src/sample_programs/getc.o 0
vm run 0

Step - [114, 0, 0, 0, 0, 0, 0, 0, 1, 0]
Step - [114, 0, 0, 0, 0, 0, 0, 0, 1, 3]
Final - [114, 0, 0, 0, 0, 0, 0, 0, 1, 3]
```

### Example with interrupts -
![vm_ic.png](vm_ic.png)


# References
- [https://en.wikipedia.org/wiki/Little_Computer_3](https://en.wikipedia.org/wiki/Little_Computer_3)
- [https://en.wikipedia.org/wiki/Little_man_computer](https://en.wikipedia.org/wiki/Little_man_computer)
- [https://www.jmeiners.com/lc3-vm/#:lc3.c_2](https://www.jmeiners.com/lc3-vm/#:lc3.c_2)
- [https://www.youtube.com/watch?v=oArXOAhzOdY&list=PLUkZG7_4JtUL22HycWYR_J-1xJo7rQGhr](https://www.youtube.com/watch?v=oArXOAhzOdY&list=PLUkZG7_4JtUL22HycWYR_J-1xJo7rQGhr)
- [https://www.andreinc.net/2021/12/01/writing-a-simple-vm-in-less-than-125-lines-of-c](https://www.andreinc.net/2021/12/01/writing-a-simple-vm-in-less-than-125-lines-of-c)

# Overview
Implementation of 16-bit virtual machine (VM).

# Implementation
## What value and API does a VM provide?
- A virtual machine is a mock for a real machine, in our case a CPU.
- The main value that CPU provides is to run instructions.
- We would follow the LC-3 architecture of a CPU to build our virtual machine.
- There are some assumptions and pre-requisites that need to be filled to be able to run a program (a set of instructions).
- The machine would expose a component called "Memeory" where the programmer must load their program instructions.
- Once loaded, programmer would start the machine.
- If booted correctly, the machine will start executing and stepping through each instruction of the program one at a time.
- The instruction itself would be an Assembly instruction as the language instruction set is smaller and easier to implement.

## How does it work internally?
- Other than the memory, machine needs some components with special meaning to control the flow of execution, handle intermediate results and provide final result of a running program.
- These special components are modelled as "Registers".
- For the purpose of LC-3 architecture, we need 8 registers. (more details on what and why here (pending)).

# References
- https://www.jmeiners.com/lc3-vm/#:lc3.c_2
- https://www.youtube.com/watch?v=oArXOAhzOdY&list=PLUkZG7_4JtUL22HycWYR_J-1xJo7rQGhr
- https://www.andreinc.net/2021/12/01/writing-a-simple-vm-in-less-than-125-lines-of-c

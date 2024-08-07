use vmrs::Machine;

fn main() {
    let mut vm = Machine::default();
    let program_code: [u16; 4] = [
        0b0010000000000011, // Load R0 3
        0b0010001000000110, // Load R1 6
        0b0001010000000001, // Add R2 R0 R1
        0b1111000000000000, // Trap #for halt
    ];
    vm.run(0, program_code.as_slice()).unwrap();
}

use std::io::stdin;

use vmrs::Machine;

fn main() {
    let mut vm = Machine::default();
    let mut program_code = vec![];
    for line in stdin().lines() {
        let Ok(line) = line else { continue };
        let instr = line.split('#').collect::<Vec<&str>>()[0].replace(' ', "");
        program_code.push(u16::from_str_radix(instr.as_str(), 2).unwrap())
    }
    vm.run(0, program_code.as_slice()).unwrap();
    println!("Final - {:?}", vm.dump().registers);
}

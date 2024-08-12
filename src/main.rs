use std::io::stdin;

use vmrs::Machine;

fn main() {
    let mut vm = Machine::default();
    let mut program_code = vec![];
    for line in stdin().lines() {
        let Ok(line) = line else { break };
        if line.is_empty() {
            break;
        }
        let instr = line.split('#').collect::<Vec<&str>>()[0].replace(' ', "");
        program_code.push(u16::from_str_radix(instr.as_str(), 2).unwrap())
    }
    if !program_code.is_empty() {
        vm.run(0, program_code.as_slice()).unwrap();
    }
    println!("Final - {:?}", vm.dump().registers);
}

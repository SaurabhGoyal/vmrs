use std::{env, fs, io::stdin};

use vmrs::Machine;

fn main() {
    let mut vm = Machine::default();
    for cmd in stdin().lines() {
        let Ok(cmd) = cmd else { break };
        if cmd.is_empty() {
            break;
        }
        let cmd_parts = cmd.split(' ').collect::<Vec<&str>>();
        if cmd_parts[0] == "load" {
            let data_file = cmd_parts[1];
            let addr = cmd_parts[2].parse::<u16>().unwrap();
            let mut data = vec![];
            for line in fs::read_to_string(data_file).unwrap().lines() {
                let data_item = line.split('#').collect::<Vec<&str>>()[0].replace(' ', "");
                data.push(u16::from_str_radix(data_item.as_str(), 2).unwrap());
            }
            vm.load(addr, data.as_slice()).unwrap();
            println!("Load - {:?}", vm.dump());
        } else if cmd_parts[0] == "run" {
            let addr = cmd_parts[1].parse::<u16>().unwrap();
            vm.run(addr).unwrap();
            println!("Final - {:?}", vm.dump());
        } else {
            println!("Invalid cmd!");
            break;
        }
    }
}

use std::{fs, io::stdin, sync::mpsc::channel, thread};

use vmrs::Machine;

fn main() {
    let (vm_tx, vm_rx) = channel::<Vec<String>>();
    let machine_handler = thread::spawn(move || {
        let mut vm = Machine::default();
        while let Ok(cmd_parts) = vm_rx.recv() {
            if cmd_parts[0] == "load" {
                let data_file = cmd_parts[1].as_str();
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
    });

    let input_handler = thread::spawn(move || {
        for cmd in stdin().lines() {
            let Ok(cmd) = cmd else { break };
            if cmd.is_empty() {
                break;
            }
            println!("checking {cmd}");
            let mut cmd_parts = cmd
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let cmd_target = cmd_parts.remove(0);
            if cmd_target == "vm" {
                vm_tx.send(cmd_parts).unwrap();
            } else {
                println!("Invalid command!");
            }
        }
    });
    machine_handler.join().unwrap();
    input_handler.join().unwrap();
}

use std::{
    fs,
    io::stdin,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};
use tracing::{event, instrument, Level};

use vmrs::Machine;

fn main() {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();
    let (vm_tx, vm_rx) = channel::<Vec<String>>();
    let machine_handler = thread::spawn(move || vm_runner(vm_rx));
    let shell_handler = thread::spawn(move || shell_runner(vm_tx));
    machine_handler.join().unwrap();
    shell_handler.join().unwrap();
}

#[instrument]
fn vm_runner(cmd_recvr: Receiver<Vec<String>>) {
    let mut vm = Machine::default();
    while let Ok(cmd_parts) = cmd_recvr.recv() {
        if cmd_parts[0] == "load" {
            let data_file = cmd_parts[1].as_str();
            let addr = cmd_parts[2].parse::<u16>().unwrap();
            let mut data = vec![];
            for line in fs::read_to_string(data_file).unwrap().lines() {
                let data_item = line.split('#').collect::<Vec<&str>>()[0].replace(' ', "");
                data.push(u16::from_str_radix(data_item.as_str(), 2).unwrap());
            }
            vm.load(addr, data.as_slice()).unwrap();
            event!(Level::INFO, "Load - {:?}", vm.dump());
        } else if cmd_parts[0] == "set_pc" {
            let addr = cmd_parts[1].parse::<u16>().unwrap();
            vm.set_pc(addr).unwrap();
            event!(Level::INFO, "Set_PC - {:?}", vm.dump());
        } else if cmd_parts[0] == "next" {
            vm.cycle().unwrap();
            event!(Level::INFO, "Cycle - {:?}", vm.dump());
        } else {
            event!(Level::ERROR, "Invalid cmd!");
            break;
        }
    }
}

#[instrument]
fn shell_runner(vm_tx: Sender<Vec<String>>) {
    for cmd in stdin().lines() {
        let Ok(cmd) = cmd else { break };
        if cmd.is_empty() {
            break;
        }
        let mut cmd_parts = cmd
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let cmd_target = cmd_parts.remove(0);
        if cmd_target == "vm" {
            vm_tx.send(cmd_parts).unwrap();
        } else {
            event!(Level::INFO, "Invalid command!");
        }
    }
}

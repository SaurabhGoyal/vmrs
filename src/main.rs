use std::{
    fs,
    io::stdin,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};
use tracing::{event, instrument, Level};

use vmrs::{InterruptController, Machine, RSTAT_HALT, RSTAT_WAITING_FOR_INPUT};

fn main() {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();
    let (vm_tx, vm_rx) = channel::<Vec<String>>();
    let (ic_tx, ic_rx) = channel::<Vec<String>>();
    let vm_tx_clone = vm_tx.clone();
    let ic_tx_clone = ic_tx.clone();
    let machine_handler = thread::spawn(move || vm_runner(vm_rx, ic_tx_clone));
    let ic_handler = thread::spawn(move || ic_runner(ic_rx, vm_tx_clone));
    let shell_handler = thread::spawn(move || shell_runner(vm_tx, ic_tx));
    machine_handler.join().unwrap();
    ic_handler.join().unwrap();
    shell_handler.join().unwrap();
}

#[instrument(skip_all)]
fn vm_runner(cmd_recvr: Receiver<Vec<String>>, ic_sender: Sender<Vec<String>>) {
    let mut vm = Machine::default();
    while let Ok(cmd_parts) = cmd_recvr.recv() {
        if cmd_parts[0] == "load" {
            let data_file = cmd_parts[1].as_str();
            let segment = cmd_parts[2].parse::<u8>().unwrap();
            let addr = cmd_parts[3].parse::<u16>().unwrap();
            let mut data = vec![];
            for line in fs::read_to_string(data_file).unwrap().lines() {
                let data_item = line.split('#').collect::<Vec<&str>>()[0].replace(' ', "");
                data.push(u16::from_str_radix(data_item.as_str(), 2).unwrap());
            }
            vm.load(segment, addr, data.as_slice()).unwrap();
            event!(Level::INFO, command = "load", "{:?}", vm.dump().memory);
        } else if cmd_parts[0] == "set_pc" {
            let addr = cmd_parts[1].parse::<u16>().unwrap();
            vm.set_pc(addr).unwrap();
            event!(Level::INFO, command = "set_pc", "{:?}", vm.dump().registers);
        } else if cmd_parts[0] == "exec" {
            let rstat = vm.execute_instruction().unwrap();
            if !matches!(rstat, RSTAT_HALT | RSTAT_WAITING_FOR_INPUT) {
                event!(Level::INFO, command = "exec", "{:?}", vm.dump().registers);
            } else {
                event!(
                    Level::WARN,
                    command = "exec",
                    "Program halted or waiting for input!\n{:?}",
                    vm.dump().registers
                );
            }
        } else if cmd_parts[0] == "int" {
            let dev_id = cmd_parts[1].parse::<u8>().unwrap();
            let int_id = cmd_parts[2].parse::<u16>().unwrap();
            vm.handle_interrupt(dev_id, int_id).unwrap();
            event!(Level::INFO, command = "int", "{:?}", vm.dump().registers);
            ic_sender
                .send(vec!["int_ack".to_string(), int_id.to_string()])
                .unwrap();
        } else {
            event!(Level::ERROR, "Invalid cmd!");
            break;
        }
    }
}

#[instrument(skip_all)]
fn ic_runner(cmd_recvr: Receiver<Vec<String>>, vm_sender: Sender<Vec<String>>) {
    let mut ic = InterruptController::default();
    while let Ok(cmd_parts) = cmd_recvr.recv() {
        if cmd_parts[0] == "int" {
            let dev_id = cmd_parts[1].parse::<u8>().unwrap();
            let int_id = cmd_parts[2].parse::<u16>().unwrap();
            ic.int(dev_id, int_id).unwrap();
            event!(Level::INFO, command = "int", "{:?}", ic);
            vm_sender.send(cmd_parts.clone()).unwrap();
        } else if cmd_parts[0] == "int_ack" {
            let int_id = cmd_parts[1].parse::<u16>().unwrap();
            ic.int_ack(int_id).unwrap();
            event!(Level::INFO, command = "int_ack", "{:?}", ic);
        } else {
            event!(Level::ERROR, "Invalid cmd!");
            break;
        }
    }
}

#[instrument(skip_all)]
fn shell_runner(vm_tx: Sender<Vec<String>>, ic_tx: Sender<Vec<String>>) {
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
        } else if cmd_target == "ic" {
            ic_tx.send(cmd_parts).unwrap();
        } else {
            event!(Level::INFO, "Invalid command!");
        }
    }
}

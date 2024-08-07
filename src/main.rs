use vmrs::Machine;

fn main() {
    let vm = Machine::default();
    println!("Machine initialised - {:?}", vm.dump());
}

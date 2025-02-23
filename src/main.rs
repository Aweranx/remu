use std::env;
use crate::machine::Machine;

mod elfdef;
mod remu;
mod mmu;
mod machine;

fn main() {
    let mut machine: Machine = Machine::default();
    let args: Vec<String> = env::args().collect();
    match machine.load_program(&args[1]) {
        Ok(()) => {
            println!("host_alloc: {:x}",machine.mmu.host_alloc);
        }
        Err(e) => {
            eprintln!("Error loading program: {}", e);
            std::process::exit(1);
        }
    }
}

use std::fs::File;
use std::path::Path;
use crate::mmu::Mmu;
use goblin::error::Error;
/*
    register group & pc
 */
#[derive(Debug, Default)]
pub struct State {
    pub gp_regs: [u64; 32],
    pub pc: u64,
}

/*
    the cpu
 */
#[derive(Debug, Default)]
pub struct Machine {
    pub state: State,
    pub mmu: Mmu,
}

impl Machine {
    pub fn load_program(&mut self, program: &str) ->Result<(), Error> {
        let path = Path::new(program);
        let mut file = File::open(path)?;
        self.mmu.load_elf(file)?;
        Ok(())
    }
}
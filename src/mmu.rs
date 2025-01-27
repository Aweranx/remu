use std::fs::File;
use std::io::{ ErrorKind, Read, Seek, SeekFrom};
use goblin::{elf, elf::Elf, elf::program_header::ProgramHeader, elf::program_header::PT_LOAD};
use goblin::error::Error;
use crate::elfdef::{EI_CLASS, EI_NIDENT, ELFCLASS64, ELFMAG, EM_RISCV};

/*
    mange memory unit
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct Mmu {
    pub entry: u64,
    pub host_alloc: u64,
    pub alloc: u64,
    pub base: u64,
}
impl Mmu {
    pub fn load_elf(&mut self, mut file: File) ->Result<(), Error> {
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // 解析 ELF 文件
        let elf = Elf::parse(&buffer)?;
        // // check the ELF magic number
        if elf.header.e_ident[..4] != ELFMAG[..] {
            return Err(Error::BadMagic(2135247942));
        }
        // check the machine flag
        if elf.header.e_machine != EM_RISCV || elf.header.e_ident[EI_CLASS] != ELFCLASS64 {
            return Err(Error::Malformed("only riscv64 is supported".to_owned()));
        }

        self.entry = elf.header.e_entry;

        // load the segments
        let loadable_segments: Vec<&ProgramHeader> = elf.program_headers
            .iter()
            .filter(|ph| ph.p_type == PT_LOAD)
            .collect();
        for segment in loadable_segments {
            println!("  Offset: 0x{:x}", segment.p_offset);
            println!("  Virtual Address: 0x{:x}", segment.p_vaddr);
            println!("  File Size: {}", segment.p_filesz);
            println!("  Memory Size: {}", segment.p_memsz);
        }
        Ok(())
    }




}
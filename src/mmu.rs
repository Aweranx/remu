use std::cmp::max;
use std::fs::File;
use std::io::{ ErrorKind, Read, Seek, SeekFrom};
use goblin::{elf, elf::Elf, elf::program_header::ProgramHeader, elf::program_header::PT_LOAD};
use goblin::error::Error;
use nix::libc::mmap;
use nix::sys::mman::{MapFlags, ProtFlags};
use nix::unistd;
use crate::elfdef::{EI_CLASS, EI_NIDENT, ELFCLASS64, ELFMAG, EM_RISCV, PF_R, PF_W, PF_X};
use crate::remu::{round_down, round_up, to_host, to_guest};

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
        let page_size = Self::get_page_size();

        let loadable_segments: Vec<&ProgramHeader> = elf.program_headers
            .iter()
            .filter(|ph| ph.p_type == PT_LOAD)
            .collect();
        for segment in loadable_segments {
            let offset = segment.p_offset;
            let vaddr = to_host(segment.p_vaddr);
            let aligned_vaddr = round_down(vaddr, page_size);
            let filesz = segment.p_filesz + (vaddr - aligned_vaddr);
            let memsz = segment.p_memsz + (vaddr - aligned_vaddr);
            let prot = flags_to_mmap_prot(segment.p_flags);
            let flags = MapFlags::MAP_PRIVATE | MapFlags::MAP_FIXED;

            let mut addr: *mut core::ffi::c_void = unsafe { mmap(aligned_vaddr as *mut core::ffi::c_void,
                                                             filesz as libc::size_t,
                                                             prot.bits() as libc::c_int,
                                                             flags.bits() as libc::c_int ,
                                                             -1,
                                                             round_down(offset, page_size) as libc::off_t)};

            let remaining_bss = round_up(memsz, page_size) - round_up(filesz, page_size);
            if(remaining_bss > 0) {
                addr = unsafe { mmap((aligned_vaddr + round_up(filesz, page_size)) as *mut core::ffi::c_void,
                                     filesz as libc::size_t,
                                     prot.bits() as libc::c_int,
                                     flags.bits() as libc::c_int ,
                                     -1,
                                     round_down(offset, page_size) as libc::off_t)};
            }

            self.host_alloc = max(self.host_alloc, aligned_vaddr + round_up(memsz, page_size));
            self.base = to_guest(self.host_alloc);
            self.alloc = self.base;

        }
        Ok(())
    }

    fn get_page_size() -> u64 {
        match unistd::sysconf(unistd::SysconfVar::PAGE_SIZE) {
            Ok(page_size_option) => {
                match page_size_option {
                    Some(page_size_clong) => page_size_clong as u64,
                    None => 0,
                }
            },
            Err(_) => 0,
        }
    }



}

fn flags_to_mmap_prot(flags: u32) -> ProtFlags {
    let mut prot = ProtFlags::empty();

    if flags & PF_R != 0 {
        prot |= ProtFlags::PROT_READ;
    }
    if flags & PF_W != 0 {
        prot |= ProtFlags::PROT_WRITE;
    }
    if flags & PF_X != 0 {
        prot |= ProtFlags::PROT_EXEC;
    }

    prot
}
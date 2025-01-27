use std::fmt;

// Define constants for ELF file identification
pub const EI_NIDENT: usize = 16;
pub const ELFMAG: &[u8; 4] = b"\x7FELF"; // Use byte string literal for magic number

// Define the machine type for RISC-V
pub const EM_RISCV: u16 = 243;

// Define ELF class constants
pub const EI_CLASS: usize = 4;
pub const ELFCLASSNONE: u8 = 0;
pub const ELFCLASS32: u8 = 1;
pub const ELFCLASS64: u8 = 2;
pub const ELFCLASSNUM: u8 = 3;


// Define program header flags
pub const PF_X: u32 = 0x1;
pub const PF_W: u32 = 0x2;
pub const PF_R: u32 = 0x4;
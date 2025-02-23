## goblin::elf::header::header64::Header

* e_ident: [u8; 16]
    - Magic number and other info

* e_type: u16
    - Object file type

* e_machine: u16
    - Architecture

* e_version: u32
    - Object file version

* e_entry: u64
    - Entry point virtual address

* e_phoff: u64
    - Program header table file offset

* e_shoff: u64
    - Section header table file offset

* e_flags: u32
    - Processor-specific flags

* e_ehsize: u16
    - ELF header size in bytes

* e_phentsize: u16
    - Program header table entry size

* e_phnum: u16
    - Program header table entry count

* e_shentsize: u16
    - Section header table entry size

* e_shnum: u16
    - Section header table entry count

* e_shstrndx: u16
    - Section header string table index

## goblin::elf::program_header::program_header64::ProgramHeader

* p_type: u32
    - Segment type

* p_flags: u32
    - Segment flags

* p_offset: u64
    - Segment file offset

* p_vaddr: u64
    - Segment virtual address

* p_paddr: u64
    - Segment physical address

* p_filesz: u64
    - Segment size in file

* p_memsz: u64
    - Segment size in memory

* p_align: u64
    - Segment alignment
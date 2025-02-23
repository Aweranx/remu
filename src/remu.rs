
pub const GUEST_MEMORY_OFFSET: u64 = 0x088800000000;

pub fn round_down(x: u64, k: u64) -> u64 {
    x & !k
}

pub fn round_up(x: u64, k: u64) -> u64 {
    (x + k - 1) & !k
}

pub fn to_host(addr: u64) -> u64 {
    addr.wrapping_add(GUEST_MEMORY_OFFSET)
}

pub fn to_guest(addr: u64) -> u64 {
    addr.wrapping_sub(GUEST_MEMORY_OFFSET)
}
pub fn is_aligned(addr: usize, alignment: usize) -> bool {
    (addr & !(alignment - 1)) == addr
}

pub fn pairwise_swap(x: u32) -> u32 {
    let odd_bits = x & 0xAAAAAAAA;   
    let even_bits = x & 0x55555555; 

    (odd_bits >> 1) | (even_bits << 1)
}

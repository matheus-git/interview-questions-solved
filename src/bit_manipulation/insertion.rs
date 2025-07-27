pub fn insert_bits(n: u32, m: u32, i: u8, j: u8) -> u32 {
    let all_ones = !0u32;

    let left = all_ones << (j + 1);
    let right = (1 << i) - 1;
    let mask = left | right;

    let n_cleared = n & mask;
    let m_shifted = m << i;

    n_cleared | m_shifted
}

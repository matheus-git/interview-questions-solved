pub fn bit_flips_needed(a: u32, b: u32) -> u32 {
    let mut count = 0;
    let mut diff = a ^ b; 

    while diff != 0 {
        count += diff & 1;
        diff >>= 1;      
    }

    count
}


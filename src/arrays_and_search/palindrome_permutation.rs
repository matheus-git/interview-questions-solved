pub fn is_permutation_of_palindrome(phrase: &str) -> bool {
    fn toggle(bit_vector: usize, index: usize) -> usize {
        let mask = 1 << index;
        if bit_vector & mask == 0 {
            bit_vector | mask
        } else {
            bit_vector & !mask
        }
    }

    fn create_bit_vector(phrase: &str) -> usize {
        let mut bit_vector: usize = 0;
        for c in phrase.chars() {
            if c.is_ascii_alphabetic() {
                let idx = (c.to_ascii_lowercase() as u8 - b'a') as usize;
                bit_vector = toggle(bit_vector, idx);
            }
        }
        bit_vector
    }

    fn check_at_most_one_bit_set(bit_vector: usize) -> bool {
        bit_vector == 0 || (bit_vector & (bit_vector - 1)) == 0
    }

    let bit_vector = create_bit_vector(phrase);
    check_at_most_one_bit_set(bit_vector)
}

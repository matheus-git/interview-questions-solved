pub fn is_unique(s: &str) -> bool {
    if s.len() > 128 {
        return false;
    }

    let mut char_set = [false; 128];
    for b in s.bytes() {
        let idx = b as usize;
        if char_set[idx] {
            return false;
        }
        char_set[idx] = true;
    }
    true
}

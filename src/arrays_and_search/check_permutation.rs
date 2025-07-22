pub fn sort_the_strings(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    fn sort(s: &str) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        chars.into_iter().collect()
    }

    sort(s1) == sort(s2)
}

pub fn strings_have_identical_character_counts(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut letters: [i8; 128] = [0; 128];
    for b in s1.bytes() {
        letters[b as usize] += 1;
    }

    for b in s2.bytes() {
        letters[b as usize] -= 1;
        if letters[b as usize] < 0 {
            return false;
        }
    }

    true
}

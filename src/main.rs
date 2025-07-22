mod arrays_and_search;

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrays_and_search::*;

    #[test]
    fn test_unique_chars() {
        assert_eq!(is_unique::is_unique("abcde"), true);
        assert_eq!(is_unique::is_unique("AaBbCc"), true);
        assert_eq!(is_unique::is_unique("test"), false);
    }

    #[test]
    fn test_check_permutation() {
        assert_eq!(check_permutation::sort_the_strings("listen", "silent"), true);
        assert_eq!(check_permutation::sort_the_strings("apple", "aplee"), false);

        assert_eq!(check_permutation::strings_have_identical_character_counts("listen", "silent"), true);
        assert_eq!(check_permutation::strings_have_identical_character_counts("apple", "aplee"), false);
    }
}

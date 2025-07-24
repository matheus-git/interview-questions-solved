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

    #[test]
    fn test_urlify() {
        assert_eq!(urlify::urlify("Mr John Smith   "), "Mr%20John%20Smith");

        let mut s: Vec<char> = "Mr John Smith    ".chars().collect();
        urlify::urlify_in_place(&mut s, 13);
        let s: String = s.into_iter().collect();
        assert_eq!(s, "Mr%20John%20Smith");
    }

    #[test]
    fn test_palindrome_permutation() {
        assert_eq!(palindrome_permutation::is_permutation_of_palindrome("Tact Coa"), true);
        assert_eq!(palindrome_permutation::is_permutation_of_palindrome("hello"), false);
    }
}

mod arrays_and_search;
mod stack_and_queues;

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrays_and_search::*;
    use stack_and_queues::*;

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

    #[test]
    fn test_one_away(){
        assert_eq!(one_away::one_edit_away("pale","ple"), true);
        assert_eq!(one_away::one_edit_away("pale","bae"), false);
    }

    #[test]
    fn test_string_compression(){
        assert_eq!(string_compression::compress("aabcccccaaa".to_string()), "a2b1c5a3".to_string());
        assert_eq!(string_compression::compress("tomato".to_string()), "tomato".to_string());
    }

    #[test]
    fn test_sort_stack(){
        assert_eq!(sort_stack::sort_stack(vec![4,5,3,2,8,1]), vec![8,5,4,3,2,1]);
    }
}

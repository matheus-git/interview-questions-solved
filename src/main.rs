mod arrays_and_search;
mod stack_and_queues;
mod bit_manipulation;

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrays_and_search::*;
    use stack_and_queues::*;
    use bit_manipulation::*;

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

    #[test]
    fn test_insertion(){
        let n = 0b10000000000;
        let m = 0b10011;
        let i = 2;
        let j = 6;

        let result = insertion::insert_bits(n, m, i, j);
        assert_eq!(result, 0b10001001100);
    }

    #[test]
    fn test_binary_to_string(){
        assert_eq!(binary_to_string::binary_to_string(0.625), "0.101".to_string());
    }

    #[test]
    fn test_flip_bit_to_win(){
        assert_eq!(flip_bit_to_win::flip_bit_to_win(1775), 8);
    }

    #[test]
    fn test_bit_flips_needes(){
        assert_eq!(conversions::bit_flips_needed(29, 15), 2);
    }

    #[test]
    fn test_pairwise_swap(){
        assert_eq!(pairwise_swap::pairwise_swap(178), 113);
    }
}   

mod arrays_and_search;


fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_chars() {
        assert_eq!(arrays_and_search::is_unique::is_unique("abcde"), true);
        assert_eq!(arrays_and_search::is_unique::is_unique("AaBbCc"), true);
        assert_eq!(arrays_and_search::is_unique::is_unique("test"), false);
    }
}

pub fn compress(s: String) -> String {
    fn count_compression(s: &str) -> usize {
        if s.is_empty() {
            return 0;
        }

        let mut compressed_length = 0;
        let mut count_consecutive = 1;
        let mut chars = s.chars().peekable();

        let mut current = chars.next();

        while let Some(c) = current {
            while let Some(&next) = chars.peek() {
                if next == c {
                    count_consecutive += 1;
                    chars.next();
                } else {
                    break;
                }
            }
            compressed_length += 1 + count_consecutive.to_string().len();
            count_consecutive = 1;
            current = chars.next();
        }

        compressed_length
    }

    let final_length = count_compression(&s);
    if final_length >= s.len() {
        return s.to_string();
    }
    let mut compressed = String::with_capacity(final_length);
    let mut count_consecutive = 1;

    let mut chars = s.chars().peekable();
    let mut current = chars.next();

    while let Some(c) = current {
        while let Some(&next) = chars.peek() {
            if next == c {
                count_consecutive += 1;
                chars.next();
            } else {
                break;
            }
        }

        compressed.push(c);
        compressed.push_str(&count_consecutive.to_string());
        count_consecutive = 1;

        current = chars.next();
    }

    compressed
}

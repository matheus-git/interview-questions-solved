pub fn one_edit_away(first: &str, second: &str) -> bool {
    let len1 = first.len();
    let len2 = second.len();

    if (len1 as isize - len2 as isize).abs() > 1 {
        return false;
    }

    let (s1, s2) = if len1 < len2 { (first, second) } else { (second, first) };

    let mut iter1 = s1.chars();
    let mut iter2 = s2.chars();
    let mut found_difference = false;

    let mut c1 = iter1.next();
    let mut c2 = iter2.next();

    while let (Some(ch1), Some(ch2)) = (c1, c2) {
        if ch1 != ch2 {
            if found_difference {
                return false;
            }
            found_difference = true;

            if len1 == len2 {
                c1 = iter1.next();
            }
        } else {
            c1 = iter1.next();
        }
        c2 = iter2.next();
    }

    true
}

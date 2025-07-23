pub fn urlify(s: &str) -> String {
    let max_index = s
        .char_indices()
        .rev()
        .find(|&(_, c)| c != ' ')
        .map(|(idx, _ )| idx + 1)
        .unwrap_or(0);

    let mut space_count = 0;
    for (i, c) in s.char_indices() {
        if i >= max_index {
            break;
        }
        if c == ' ' {
            space_count += 1;
        }
    }

    let mut result: String = String::with_capacity(s[..max_index].len() + space_count * 2);

    for c in s[..max_index].chars(){
        if c == ' ' {
            result.push_str("%20");
        } else{
            result.push(c);
        }
    }

    result
}

pub fn urlify_in_place(buffer: &mut Vec<char>, true_length: usize) {
    let mut last_non_space = 0;
    for i in (0..true_length).rev() {
        if buffer[i] != ' ' {
            last_non_space = i + 1; 
            break;
        }
    }

    let space_count = buffer[..last_non_space].iter().filter(|&&c| c == ' ').count();

    let new_length = last_non_space + space_count * 2;
    buffer.resize(new_length, '\0');

    let mut i = last_non_space as isize - 1;
    let mut j = new_length as isize - 1;

    while i >= 0 {
        if buffer[i as usize] == ' ' {
            buffer[j as usize] = '0';
            buffer[j as usize - 1] = '2';
            buffer[j as usize - 2] = '%';
            j -= 3;
        } else {
            buffer[j as usize] = buffer[i as usize];
            j -= 1;
        }
        i -= 1;
    }
}

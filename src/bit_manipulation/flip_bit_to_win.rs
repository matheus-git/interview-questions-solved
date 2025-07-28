pub fn flip_bit_to_win(mut num: i32) -> i32 {
    if !num == 0 {
        return 32;
    }

    let mut current_length = 0;
    let mut previous_length = 0;
    let mut max_length = 1; 

    while num != 0 {
        if (num & 1) == 1 {
            current_length += 1;
        } else if (num & 1) == 0 {
            previous_length = if (num & 2) == 0 { 0 } else { current_length };
            current_length = 0;
        }
        max_length = max_length.max(previous_length + current_length + 1);
        num >>= 1;
    }

    max_length
}

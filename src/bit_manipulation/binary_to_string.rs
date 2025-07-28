pub fn binary_to_string(mut num: f64) -> String {
    if num <= 0.0 || num >= 1.0 {
        return "ERROR".to_string();
    }

    let mut binary = String::from("0.");
    while num > 0.0 {
        if binary.len() >= 34 {
            return "ERROR".to_string();
        }

        num *= 2.0;
        if num >= 1.0 {
            binary.push('1');
            num -= 1.0;
        } else {
            binary.push('0');
        }
    }

    binary
}

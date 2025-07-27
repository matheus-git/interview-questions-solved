pub fn sort_stack(mut s: Vec<i32>) -> Vec<i32>{
    let mut r: Vec<i32> = Vec::with_capacity(s.len());

    while !s.is_empty() {
        let tmp = s.pop().unwrap();
        while !r.is_empty() && *r.last().unwrap() < tmp  {
            s.push(r.pop().unwrap());
        }
        r.push(tmp);
    }
    r
}

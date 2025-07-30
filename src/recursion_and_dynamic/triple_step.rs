pub fn count_ways(n: usize) -> u64 {
    let mut memo: Vec<u64> = vec![u64::MAX; n + 1];
    fn count_ways_recursive(n: usize, memo: &mut Vec<u64>) -> u64 {
        if n < 4 {
            return n as u64;
        } else if let Some(v) = memo.get(n) {
            if *v != u64::MAX {
                return *v;
            }
        } 

        memo[n] = count_ways_recursive(n - 1, memo) + count_ways_recursive(n - 2, memo) + count_ways_recursive(n - 3, memo);
        memo[n]
    }
    count_ways_recursive(n, &mut memo)
}


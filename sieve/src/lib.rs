pub fn primes_up_to(upper_bound: usize) -> Vec<usize> {
    let mut mark = vec![false; upper_bound + 1];
    (2..=upper_bound)
        .filter(|&n| {
            if mark[n] {
                return false;
            }
            (n..=(upper_bound / n)).for_each(|p| {
                mark[n * p] = true;
            });
            true
        })
        .collect()
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit)
        .filter(|n| factors.iter().any(|&x| (x != 0) && (n % x == 0)))
        .sum::<u32>()
}

fn sqrt(n: u64) -> u64{
    ((n as f64).sqrt() as u64)+1
}

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut mark = std::iter::repeat(true)
        .take((upper_bound+1) as usize)
        .collect::<Vec<bool>>();

    let sqrt_bound = sqrt(upper_bound);
    let half_bound = upper_bound/2;
    (2..=sqrt_bound).for_each(|n| {
        (n..=half_bound).for_each(|p| {
            if let Some(is_prime) = mark.get_mut((n * p) as usize) {
                *is_prime = false
            }
        })
    });

    mark.iter()
        .enumerate()
        .skip(2)
        .filter(|(_i, &is_prime)| is_prime)
        .map(|(i, _is_prime)| i as u64)
        .collect::<Vec<u64>>()
}

pub fn nth(n: u32) -> u32 {
    let mut number = 2;
    let mut primes : Vec<u32>= Vec::new();

    while primes.len() < ((n+1) as usize) {
        if is_prime(number, &primes) {
            primes.push(number);
        }
        number+=1;
    }

    if let Some(last) = primes.last() {
        return last.clone();
    }

    panic!("Should never be here!");
}

fn is_prime(n: u32, primes : &Vec<u32>) -> bool {
    if primes.is_empty() {
        return true;
    }

    for prime in primes {
        if n % prime == 0 {
            return false
        }
    }

    return true;
}
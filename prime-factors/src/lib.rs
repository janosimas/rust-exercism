fn _factors(number: u64, v: &mut Vec<u64>) {
    if number == 1 || number == 0 {
        return;
    }

    for factor in 2..=number {
        if number % factor == 0 {
            v.push(factor);
            _factors(number / factor, v);
            break;
        }
    }
}

fn get_factor(number: u64) -> Option<u64> {
    if number == 1 || number == 0 {
        return None;
    }

    for factor in 2..=number {
        if number % factor == 0 {
            return Some(factor);
        }
    }

    panic!();
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut v: Vec<u64> = vec![];
    let mut val = n;
    while val != 1 && val != 0 {
        let factor = get_factor(val);
        match factor {
            Some(f) => {
                v.push(f);
                val /= f;
            }
            _ => continue,
        }
    }
    v
}

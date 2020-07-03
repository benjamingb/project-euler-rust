// Problem 3
// Largest prime factor
fn prime1(mut limit: u64) -> u64 {
    let mut factor = 2;
    while limit != factor {
        if limit % factor == 0 {
            limit = limit / factor;
        } else {
            factor += 1;
        }
    }

    factor
}

pub fn solve() {
    println!("Largest prime factor is: {}", prime1(600_851_475_143))
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIMIT: u64 = 600_851_475_143;
    const RESULT: u64 = 6857;

    #[test]
    fn test_prime1() {
        assert_eq!(RESULT, prime1(LIMIT));
    }
}

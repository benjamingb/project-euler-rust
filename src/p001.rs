// Problem 1
//
// Multiples of 3 and 5
//
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
//
// Find the sum of all the multiples of 3 or 5 below 1000.
//

//Using range expression
fn solution1(limit: u32) -> u32 {
    (2..limit).filter(|&n| (n % 3) == 0 || n % 5 == 0).sum()
}

//Using range expression
fn solution2(limit: u32) -> u32 {
    (2..limit)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |sum, x| sum + x)
}

//Using for
fn solution3(limit: u32) -> u32 {
    let mut sum = 0;
    for n in 2..limit {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    sum
}

//Usign while
fn solution4(limit: u32) -> u32 {
    let mut sum = 0;
    let mut i = 2;
    while i < limit {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
        i += 1;
    }
    sum
}

//Using Infinite loop
fn solution5(limit: u32) -> u32 {
    let mut sum = 0;
    let mut i = 2;

    loop {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
        i += 1;
        if i >= limit {
            break sum; //finalize and return sum
        }
    }
}

pub fn solve() -> Vec<u32> {
    let limit = 1000;
    vec![
        solution1(limit),
        solution2(limit),
        solution3(limit),
        solution4(limit),
        solution5(limit),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIMIT: u32 = 1000;
    const RESULT: u32 = 233168;

    #[test]
    fn test_solution1() {
        assert_eq!(RESULT, solution1(LIMIT));
    }
    #[test]
    fn test_solution2() {
        assert_eq!(RESULT, solution2(LIMIT));
    }
    #[test]
    fn test_solution3() {
        assert_eq!(RESULT, solution3(LIMIT));
    }
    #[test]
    fn test_solution4() {
        assert_eq!(RESULT, solution4(LIMIT));
    }

    #[test]
    fn test_solution5() {
        assert_eq!(RESULT, solution5(LIMIT));
    }
}

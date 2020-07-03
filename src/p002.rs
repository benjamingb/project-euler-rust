#[path = "utils.rs"]
mod utils;
// Problem 2
// Even Fibonacci numbers
/**
 fn fib(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fib1(n - 1) + fib1(n - 2),
    }
}
*/


fn fib1(limit: u32) -> u32 {
    let mut state = (1, 1);
    let mut sum = 0;
    while state.1 < limit {
        if state.1 % 2 == 0 {
            sum += state.1;
        }
        state = (state.1, state.1 + state.0);
    }
    sum
}

fn fib2(limit: u32) -> u32 {
    let mut state = (1, 1);
    let mut sum = 0;
    loop {
        if state.1 > limit {
            break sum;
        }

        if state.1 % 2 == 0 {
            sum += state.1;
        }
        state = (state.1, state.1 + state.0);
    }
}

//using std::mem::replace
fn fib3(limit: u32) -> u32 {
    std::iter::repeat_with({
        let mut state = (1, 1);
        // move closure so we dont have references to this scope
        move || {
            // calculate next fibonacci number
            let next = (state.1, state.0 + state.1);

            // update the state with the next fibonacci number
            // and get the old value
            // we only care about the first number in the pair
            // in order to get the whole fibonacci sequence
            std::mem::replace(&mut state, next).0
        }
    })
    .take_while(|&x| x < limit)
    .filter(|x| x % 2 == 0)
    .sum()
}

fn fib4(limit: u32) -> u32 {
    let mut state = (0, 1);

    let sum: u32 = std::iter::repeat_with(|| {
        state = (state.1, state.0 + state.1);
        state.0
    })
    .take_while(|&x| x < limit)
    .filter(|x| x % 2 == 0)
    .sum();

    sum
}

//using fibonacci iterator 
fn fib5(limit: u32) -> u32 {
    let sum: u32 = utils::fibonacci()
        .take_while(|&x| x < limit)
        .filter(|x| x % 2 == 0)
        .sum();
    sum
}

pub fn solve() {
    let limit = 4_000_000;
    println!("fibonacci {}", fib1(limit));
    println!("fibonacci {}", fib2(limit));
    println!("fibonacci {}", fib3(limit));
    println!("fibonacci {}", fib4(limit));
    println!("fibonacci {}", fib5(limit));
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIMIT: u32 = 4_000_000;
    const RESULT: u32 = 4_613_732;

    #[test]
    fn test_fib1() {
        assert_eq!(RESULT, fib1(LIMIT));
    }

    #[test]
    fn test_fib2() {
        assert_eq!(RESULT, fib2(LIMIT));
    }
    #[test]
    fn test_fib3() {
        assert_eq!(RESULT, fib3(LIMIT));
    }
    #[test]
    fn test_fib4() {
        assert_eq!(RESULT, fib4(LIMIT));
    }
    #[test]
    fn test_fib5() {
        assert_eq!(RESULT, fib5(LIMIT));
    }
}

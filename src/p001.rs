// Problem 1
// Find the sum of all the multiples of 3 or 5 below 1000.

//Using for
pub fn solution1(num_max: u32) -> u32 {
    let mut sum = 0;
    for n in 2..num_max {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    sum
}

//Using range expression
pub fn solution2(num_max: u32) -> u32 {
    (2..num_max)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |sum, x| sum + x)
}

//Using range expression
pub fn solution3(num_max: u32) -> u32 {
    (2..num_max).filter(|&n| n % 3 == 0 || n % 5 == 0).sum()
}

pub fn solve() {
    let num_max = 1000;
    let solutions = vec![
        solution1(num_max),
        solution2(num_max),
        solution3(num_max),
    ];

    println!("Find the sum of all the multiples of 3 or 5 below 1000. \n");

    for (index, s) in solutions.iter().enumerate() {
        println!("Solution{} - the sum is :  {}", index + 1, s);
    }
}

#[cfg(test)]
mod tests {
    const NUM_MAX: u32 = 1000;
    const RESULT: u32 = 233168;

    #[test]
    fn test_solution1() {
        assert_eq!(RESULT, super::solution1(NUM_MAX));
    }
    #[test]
    fn test_solution2() {
        assert_eq!(RESULT, super::solution2(NUM_MAX));
    }
    #[test]
    fn test_solution3() {
        assert_eq!(RESULT, super::solution3(NUM_MAX));
    }
}

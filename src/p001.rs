 // Problem 1
 // Find the sum of all the multiples of 3 or 5 below 1000.

pub fn solution1(num_max:u32) -> u32 {
    let mut sum = 0;
    for n in 2..num_max{
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    sum 
}

pub fn solution2 (num_max:u32) -> u32 {
    (2..num_max)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |sum, x| sum + x)
}

pub fn solution3(num_max:u32) -> u32{
    (2..num_max).filter(|&n| n % 3 == 0 || n % 5 == 0).sum()
}

pub fn solve(){

    let num_max = 1000;
    let solutions  = vec![
        solution1(num_max),
        solution2(num_max),
        solution3(num_max),
    ];
   
    for (index, s) in solutions.iter().enumerate(){
        println!("Solution{} - sum is :  {}", index+1, s);
    }
}  

#[cfg(test)]
mod tests {
    //use test::Bencher;

    #[test]
    fn sum_below_ten() {
        assert_eq!(23, super::solution1(10));
    }

    //#[bench]
    //fn bench_add_two(b: &mut Bencher) {
    //    b.iter(|| super::solution1(10000));
    //}
}
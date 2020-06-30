 // Problem 2
 // Even Fibonacci numbers
 

 pub fn fib1(n: u32) -> u32{
     if n <= 1 {
         return 1;
     }
     fib1(n-1) + fib1(n-2)
 } 

pub fn solve(){
    println!("fibonacci {}",fib1(5));
} 

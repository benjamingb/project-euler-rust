extern crate problems;
extern crate termion;

use std::io;
use termion::color;


fn main() {
    println!(
        "{}
.........................................
Project euler problems resolved in RUST 
       by Benjamin Gonzales B. 
..........................................
    ",
        color::Fg(color::Blue)
    );

    //problems::quizzes::list();
    /*let q = problems::quizzes::list();
    let za = &q.get(0).clone();
    let xx = 
   // println!("{:#?}", q.get(0).title);*/


    loop {
        println!(
            "{}\nPlease input the number problem :",
            color::Fg(color::Green)
        );
        let mut np = String::new();

        io::stdin().read_line(&mut np).expect("failed to read line");

        let np: u32 = match np.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        display(np);
    }
}

pub fn display(np: u32) {

    

    println!("{}", color::Fg(color::Rgb(255, 255, 255)));

    match np {
        1 => {
            println!("Find the sum of all the multiples of 3 or 5 below 1000. \n");
            for (index, s) in problems::p001::solve().iter().enumerate() {
                println!("Solution{}:  {}", index + 1, s);
            }
        }
        2 => problems::p002::solve(),
        3 => problems::p003::solve(),
        _ => println!("not found"),
    }
}

extern crate problems;
extern crate termion;

use std::io;
use termion::{color};

fn main() {
    println!(
        "{}
.........................................\r
Project euler problems resolved in RUST \r
       by Benjamin Gonzales B. \r
..........................................
    ",color::Fg(color::Blue)
    );

    loop {
        println!("{}\nPlease input the number problem :", color::Fg(color::Green));
        let mut np = String::new();

        io::stdin().read_line(&mut np).expect("failed to read line");

        let np: u32 = match np.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", color::Fg(color::Rgb(255,255,255)));
        match np {
            1 => problems::p001::solve(),
            2 => problems::p002::solve(),
            _ => println!("falta implementar"),
        }
    }
}

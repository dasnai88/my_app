use rand::Rng;
use std::{i32, io};

fn read_i32() -> i32 {
    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("read_line failed");

        match s.trim().parse::<i32>() {
            Ok(x) => return x,
            Err(__) => {
                eprintln!("Введитче целое число (i32)")
            }
        }
    }
}

fn main() {
    let mut random = rand::rng();
    let rand: i32 = random.random_range(1..=100);
    let x = read_i32();
    print!("{}", x)
}

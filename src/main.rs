use rand::Rng;
use std::io::{self, Write};

fn read_i32_or_quit() -> Option<i32> {
    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("read_line failed");

        let t = s.trim();

        if t.eq_ignore_ascii_case("q") {
            return None;
        }

        match t.parse::<i32>() {
            Ok(x) => return Some(x),
            Err(_) => eprintln!("Введите целое число (i32) или Q для выхода"),
        }
    }
}

fn main() {
    let mut rng = rand::rng();

    loop {
        let secret: i32 = rng.random_range(1..=100);
        let mut tries: i32 = 0;

        println!("Я загадал число от 1 до 100.");
        println!("У вас 7 попыток. Введите число или Q для выхода.");

        loop {
            if tries >= 7 {
                println!("Попытки закончились. Загаданное число: {}", secret);
                break;
            }

            print!("Попытка {}: ", tries + 1);
            io::stdout().flush().ok();

            let Some(x) = read_i32_or_quit() else {
                return; // выходим из программы
            };

            tries += 1;

            if x > secret {
                println!("Слишком много");
            } else if x < secret {
                println!("Слишком мало");
            } else {
                println!("Ты выиграл за {} попыток!", tries);
                break;
            }
        }

        println!("Сыграть ещё раз? (Enter = да, Q = нет)");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("read_line failed");
        if again.trim().eq_ignore_ascii_case("q") {
            break;
        }
    }
}

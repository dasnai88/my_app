use std::io;
fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    println!("Press num1");
    io::stdin().read_line(&mut num1).expect("Faile");
    println!("Press num2");
    io::stdin().read_line(&mut num2).expect("Faile");

    let data1: i16 = num1.trim().parse().expect("Faile");
    let data2: u16 = num2.trim().parse().expect("Faile");
    let res = data1 + data2 as i16;
    println!("res {}", res);
    if res == 12 && res > 10 {
        println!("Hello World!")
    }
}

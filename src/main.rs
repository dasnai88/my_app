fn main() {
    let s1 = String::new();
    let s2 = String::from("Hello");

    print!("{}", s2);

    let s3 = s1 + &s2;

    let mut word = String::new();
    word.push_str("Hello");
    word.push(" ");
    word.push_str("World");
}

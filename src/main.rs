use incremint::show::{Digits, Incremint};

fn main() {
    let prev = Digits::from(2024);
    let next = Digits::from(3024);
    println!("{}", Incremint { prev, next });
}

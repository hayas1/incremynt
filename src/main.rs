use incremint::show::{Digit, Digits, Incremint};

fn main() {
    let prev = Digits(vec![
        Digit(incremint::show::TWO),
        Digit(incremint::show::ZERO),
        Digit(incremint::show::TWO),
        Digit(incremint::show::FOUR),
    ]);
    let next = Digits(vec![
        Digit(incremint::show::THREE),
        Digit(incremint::show::ZERO),
        Digit(incremint::show::TWO),
        Digit(incremint::show::FOUR),
    ]);
    println!("{}", Incremint { prev, next });
}

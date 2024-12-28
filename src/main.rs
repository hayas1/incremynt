use incremint::show::{Digit, Digits, Incremint};

fn main() {
    let prev = Digits(vec![
        Digit(incremint::show::TWO),
        Digit(incremint::show::ZERO),
        Digit(incremint::show::TWO),
        Digit(incremint::show::FIVE),
    ]);
    let next = Digits(vec![
        Digit(incremint::show::THREE),
        Digit(incremint::show::ZERO),
        Digit(incremint::show::TWO),
        Digit(incremint::show::FIVE),
    ]);
    println!("{}", Incremint { prev, next });
}

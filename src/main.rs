use incremint::show::{Digit, Digits};

fn main() {
    let year = Digits(vec![
        Digit(incremint::show::TWO),
        Digit(incremint::show::ZERO),
        Digit(incremint::show::TWO),
        Digit(incremint::show::FIVE),
    ]);
    println!("{}", year);
}

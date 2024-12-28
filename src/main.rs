use incremint::show::Show;

fn main() {
    let mut stdout = std::io::stdout();
    let year = vec![
        incremint::show::TWO,
        incremint::show::ZERO,
        incremint::show::TWO,
        incremint::show::FIVE,
    ];
    year.show(&mut stdout).unwrap();
}

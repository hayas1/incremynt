fn main() {
    for d in incremint::show::DIGIT {
        println!(
            "{}",
            d.iter()
                .map(|x| x.iter().collect())
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}

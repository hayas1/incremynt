pub type Digit = [[char; 4]; 6];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Digits(pub Vec<Digit>);
impl From<usize> for Digits {
    fn from(ds: usize) -> Self {
        let mut digits = Vec::new();
        let mut curr = ds;
        while curr > 0 {
            digits.push(super::DIGITS[curr % 10]);
            curr /= 10;
        }
        digits.reverse();
        Digits(digits)
    }
}

impl std::fmt::Display for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..6 {
            for row in &self.0 {
                for c in row[i] {
                    write!(f, "{}", c)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Digits {
    pub fn padding(&self, p: usize) -> Self {
        if p > self.0.len() {
            let mut prefix = Digits(vec![super::SPACE; p - self.0.len()]);
            for d in &self.0 {
                prefix.0.push(d.clone());
            }
            prefix
        } else {
            self.clone()
        }
    }
}

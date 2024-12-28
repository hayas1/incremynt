#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Digit(pub [[char; 4]; 6]);
impl TryFrom<usize> for Digit {
    type Error = super::error::Error;
    fn try_from(d: usize) -> Result<Self, Self::Error> {
        match d {
            0..=9 => Ok(Digit(super::DIGITS[d])),
            o => Err(super::error::Error::Overflow(o)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Digits(pub Vec<Digit>);
impl TryFrom<usize> for Digits {
    type Error = super::error::Error;
    fn try_from(ds: usize) -> Result<Self, Self::Error> {
        let mut digits = Vec::new();
        let mut curr = ds;
        while curr > 0 {
            digits.push(Digit::try_from(curr % 10)?);
            curr /= 10;
        }
        digits.reverse();
        Ok(Digits(digits))
    }
}

impl std::fmt::Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl std::fmt::Display for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..6 {
            for row in &self.0 {
                for c in row.0[i] {
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
            let mut prefix = Digits(vec![Digit(super::SPACE); p - self.0.len()]);
            for d in &self.0 {
                prefix.0.push(d.clone());
            }
            prefix
        } else {
            self.clone()
        }
    }
}

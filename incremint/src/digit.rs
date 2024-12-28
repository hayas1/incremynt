pub type Digit = [[char; 4]; 6];
pub trait TryIntoDigit {
    type Error;
    fn try_into_digit(self) -> Result<Digit, Self::Error>;
}
impl<T: Into<usize>> TryIntoDigit for T {
    type Error = super::error::Error;
    fn try_into_digit(self) -> Result<Digit, Self::Error> {
        match self.into() {
            d @ 0..=9 => Ok(super::DIGITS[d]),
            o => Err(super::error::Error::Overflow(o)),
        }
    }
}

pub type Digits = Vec<Digit>;
pub trait IntoDigits {
    fn into_digits(self) -> Digits;
}
impl<T: Into<usize>> IntoDigits for T {
    fn into_digits(self) -> Digits {
        let mut digits = Vec::new();
        let mut curr = self.into();
        while curr > 0 {
            digits.push(super::DIGITS[curr % 10]);
            curr /= 10;
        }
        digits.reverse();
        digits
    }
}

// impl std::fmt::Display for Digits {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for i in 0..6 {
//             for row in &self.digits {
//                 for c in row[i] {
//                     write!(f, "{}", c)?;
//                 }
//             }
//             writeln!(f)?;
//         }
//         Ok(())
//     }
// }
// impl Digits {
//     pub fn padding(&self, p: usize) -> Self {
//         if p > self.digits.len() {
//             let mut prefix = Self::new(vec![super::SPACE; p - self.digits.len()], self.scale);
//             for d in &self.0 {
//                 prefix.0.push(d.clone());
//             }
//             prefix
//         } else {
//             self.clone()
//         }
//     }
// }

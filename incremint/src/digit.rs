#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Digit([[char; 4]; 6]);
impl std::ops::Deref for Digit {
    type Target = [[char; 4]; 6];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Digit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl TryFrom<usize> for Digit {
    type Error = super::error::Error;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            d @ 0..=9 => Ok(Self(super::DIGITS[d])),
            o => Err(super::error::Error::Overflow(o)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Digits(Vec<Digit>);
impl std::ops::Deref for Digits {
    type Target = Vec<Digit>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Digits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<usize> for Digits {
    fn from(value: usize) -> Self {
        let mut digits = Vec::new();
        let mut curr = value;
        while curr > 0 {
            digits.push(Digit(super::DIGITS[curr % 10]));
            curr /= 10;
        }
        digits.reverse();
        Self(digits)
    }
}

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

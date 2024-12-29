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
impl std::fmt::Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let writer = super::write::Writer::new(self.clone(), 1);
        write!(f, "{}", writer)
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
impl std::fmt::Display for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let writer = super::write::Writer::new(self.clone(), 1);
        write!(f, "{}", writer)
    }
}

impl Digits {
    pub fn padding(&mut self, p: usize) {
        if p > self.len() {
            let mut prefix = Self(vec![Digit(super::SPACE); p - self.len()]);
            std::mem::swap(self, &mut prefix);
            self.extend(prefix.0);
        }
    }
}

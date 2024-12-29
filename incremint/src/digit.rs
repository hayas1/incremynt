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
impl Digit {
    pub const SPACE: Self = Self(crate::SPACE);
    pub const ZERO: Self = Self(crate::ZERO);
    pub const ONE: Self = Self(crate::ONE);
    pub const TWO: Self = Self(crate::TWO);
    pub const THREE: Self = Self(crate::THREE);
    pub const FOUR: Self = Self(crate::FOUR);
    pub const FIVE: Self = Self(crate::FIVE);
    pub const SIX: Self = Self(crate::SIX);
    pub const SEVEN: Self = Self(crate::SEVEN);
    pub const EIGHT: Self = Self(crate::EIGHT);
    pub const NINE: Self = Self(crate::NINE);
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
        if curr == 0 {
            digits.push(Digit(super::ZERO));
        }
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
    pub fn padding(&mut self, pad: Digit, p: usize) {
        if p > self.len() {
            let mut prefix = Self(vec![pad; p - self.len()]);
            std::mem::swap(self, &mut prefix);
            self.extend(prefix.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_from_usize() {
        assert_eq!(Digit::try_from(0).unwrap(), Digit::ZERO);
        assert_eq!(Digit::try_from(1).unwrap(), Digit::ONE);
        assert_eq!(Digit::try_from(2).unwrap(), Digit::TWO);
        assert_eq!(Digit::try_from(3).unwrap(), Digit::THREE);
        assert_eq!(Digit::try_from(4).unwrap(), Digit::FOUR);
        assert_eq!(Digit::try_from(5).unwrap(), Digit::FIVE);
        assert_eq!(Digit::try_from(6).unwrap(), Digit::SIX);
        assert_eq!(Digit::try_from(7).unwrap(), Digit::SEVEN);
        assert_eq!(Digit::try_from(8).unwrap(), Digit::EIGHT);
        assert_eq!(Digit::try_from(9).unwrap(), Digit::NINE);

        assert!(matches!(
            Digit::try_from(10).unwrap_err(),
            crate::error::Error::Overflow(10)
        ));
        assert!(matches!(
            Digit::try_from(100).unwrap_err(),
            crate::error::Error::Overflow(100)
        ));
        assert!(matches!(
            Digit::try_from(1000).unwrap_err(),
            crate::error::Error::Overflow(1000)
        ));
    }

    #[test]
    fn test_digits_from_usize() {
        assert_eq!(Digits::from(0), Digits(vec![Digit::ZERO]));
        assert_eq!(Digits::from(1), Digits(vec![Digit::ONE]));
        assert_eq!(Digits::from(2), Digits(vec![Digit::TWO]));
        assert_eq!(Digits::from(3), Digits(vec![Digit::THREE]));
        assert_eq!(Digits::from(4), Digits(vec![Digit::FOUR]));
        assert_eq!(Digits::from(5), Digits(vec![Digit::FIVE]));
        assert_eq!(Digits::from(6), Digits(vec![Digit::SIX]));
        assert_eq!(Digits::from(7), Digits(vec![Digit::SEVEN]));
        assert_eq!(Digits::from(8), Digits(vec![Digit::EIGHT]));
        assert_eq!(Digits::from(9), Digits(vec![Digit::NINE]));

        assert_eq!(
            Digits::from(10),
            Digits(vec![Digit(crate::ONE), Digit(crate::ZERO)])
        );
        assert_eq!(
            Digits::from(100),
            Digits(vec![
                Digit(crate::ONE),
                Digit(crate::ZERO),
                Digit(crate::ZERO)
            ])
        );
        assert_eq!(
            Digits::from(1000),
            Digits(vec![
                Digit(crate::ONE),
                Digit(crate::ZERO),
                Digit(crate::ZERO),
                Digit(crate::ZERO)
            ])
        );
    }

    #[test]
    fn test_digit_display() {
        let digit = Digit::try_from(3).unwrap();
        println!("{}", digit);
        assert_eq!(
            digit.to_string(),
            indoc::indoc! {"
                ┏━━┓
                ┗━┓┃
                ┏━┛┃
                ┗━┓┃
                ┏━┛┃
                ┗━━┛
            "}
        );
    }

    #[test]
    fn test_digits_display() {
        let digits = Digits::from(2025);
        assert_eq!(
            digits.to_string(),
            indoc::indoc! {"
                ┏━━┓┏━━┓┏━━┓┏━━┓
                ┗━┓┃┃┏┓┃┗━┓┃┃┏━┛
                ┏━┛┃┃┃┃┃┏━┛┃┃┗━┓
                ┃┏━┛┃┃┃┃┃┏━┛┗━┓┃
                ┃┗━┓┃┗┛┃┃┗━┓┏━┛┃
                ┗━━┛┗━━┛┗━━┛┗━━┛
            "}
        );
    }

    #[test]
    fn test_digits_padding() {
        let mut digits = Digits::from(16);
        assert_eq!(
            digits.to_string(),
            vec![
                " ┏┓ ┏━━┓",
                " ┃┃ ┃┏━┛",
                " ┃┃ ┃┗━┓",
                " ┃┃ ┃┏┓┃",
                " ┃┃ ┃┗┛┃",
                " ┗┛ ┗━━┛",
                "",
            ]
            .join("\n")
        );

        digits.padding(Digit::SPACE, 4);
        assert_eq!(
            digits.to_string(),
            vec![
                "         ┏┓ ┏━━┓",
                "         ┃┃ ┃┏━┛",
                "         ┃┃ ┃┗━┓",
                "         ┃┃ ┃┏┓┃",
                "         ┃┃ ┃┗┛┃",
                "         ┗┛ ┗━━┛",
                "",
            ]
            .join("\n")
        );

        let mut luck = Digits::default();
        assert_eq!(luck.to_string(), "\n\n\n\n\n\n");

        luck.padding(Digit::SEVEN, 3);
        assert_eq!(
            luck.to_string(),
            vec![
                "┏━━┓┏━━┓┏━━┓",
                "┃┏┓┃┃┏┓┃┃┏┓┃",
                "┗┛┃┃┗┛┃┃┗┛┃┃",
                "  ┃┃  ┃┃  ┃┃",
                "  ┃┃  ┃┃  ┃┃",
                "  ┗┛  ┗┛  ┗┛",
                "",
            ]
            .join("\n")
        );
    }
}

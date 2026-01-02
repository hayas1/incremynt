use std::fmt::{Display, Formatter};

pub const ROWS: usize = 6;
pub const COLS: usize = 4;
pub type RowRepresentation = [char; COLS];
pub type DigitRepresentation = [RowRepresentation; ROWS];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum Digit {
    #[default]
    Space,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl Digit {
    pub const SPACE: DigitRepresentation = [
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' '],
    ];

    pub const ZERO: DigitRepresentation = [
        ['┏', '━', '━', '┓'],
        ['┃', '┏', '┓', '┃'],
        ['┃', '┃', '┃', '┃'],
        ['┃', '┃', '┃', '┃'],
        ['┃', '┗', '┛', '┃'],
        ['┗', '━', '━', '┛'],
    ];

    pub const ONE: DigitRepresentation = [
        [' ', '┏', '┓', ' '],
        [' ', '┃', '┃', ' '],
        [' ', '┃', '┃', ' '],
        [' ', '┃', '┃', ' '],
        [' ', '┃', '┃', ' '],
        [' ', '┗', '┛', ' '],
    ];

    pub const TWO: DigitRepresentation = [
        ['┏', '━', '━', '┓'],
        ['┗', '━', '┓', '┃'],
        ['┏', '━', '┛', '┃'],
        ['┃', '┏', '━', '┛'],
        ['┃', '┗', '━', '┓'],
        ['┗', '━', '━', '┛'],
    ];

    pub const THREE: DigitRepresentation = [
        ['┏', '━', '━', '┓'],
        ['┗', '━', '┓', '┃'],
        ['┏', '━', '┛', '┃'],
        ['┗', '━', '┓', '┃'],
        ['┏', '━', '┛', '┃'],
        ['┗', '━', '━', '┛'],
    ];

    pub const FOUR: DigitRepresentation = [
        ['┏', '┓', '┏', '┓'],
        ['┃', '┃', '┃', '┃'],
        ['┃', '┗', '┛', '┃'],
        ['┗', '━', '┓', '┃'],
        [' ', ' ', '┃', '┃'],
        [' ', ' ', '┗', '┛'],
    ];

    pub const FIVE: DigitRepresentation = [
        ['┏', '━', '━', '┓'],
        ['┃', '┏', '━', '┛'],
        ['┃', '┗', '━', '┓'],
        ['┗', '━', '┓', '┃'],
        ['┏', '━', '┛', '┃'],
        ['┗', '━', '━', '┛'],
    ];

    pub const SIX: DigitRepresentation = [
        ['┏', '━', '━', '┓'],
        ['┃', '┏', '━', '┛'],
        ['┃', '┗', '━', '┓'],
        ['┃', '┏', '┓', '┃'],
        ['┃', '┗', '┛', '┃'],
        ['┗', '━', '━', '┛'],
    ];

    pub const SEVEN: DigitRepresentation = [
        ['┏', '━', '━', '┓'],
        ['┃', '┏', '┓', '┃'],
        ['┗', '┛', '┃', '┃'],
        [' ', ' ', '┃', '┃'],
        [' ', ' ', '┃', '┃'],
        [' ', ' ', '┗', '┛'],
    ];

    pub const EIGHT: DigitRepresentation = [
        ['┏', '━', '━', '┓'],
        ['┃', '┏', '┓', '┃'],
        ['┃', '┗', '┛', '┃'],
        ['┃', '┏', '┓', '┃'],
        ['┃', '┗', '┛', '┃'],
        ['┗', '━', '━', '┛'],
    ];

    pub const NINE: DigitRepresentation = [
        ['┏', '━', '━', '┓'],
        ['┃', '┏', '┓', '┃'],
        ['┃', '┗', '┛', '┃'],
        ['┗', '━', '┓', '┃'],
        ['┏', '━', '┛', '┃'],
        ['┗', '━', '━', '┛'],
    ];
}
impl Digit {
    #[inline]
    pub const fn mod_10(n: usize) -> Self {
        match n % 10 {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            _ => unreachable!(),
        }
    }
    #[inline]
    pub const fn representation(&self) -> &DigitRepresentation {
        match self {
            Self::Space => &Self::SPACE,
            Self::Zero => &Self::ZERO,
            Self::One => &Self::ONE,
            Self::Two => &Self::TWO,
            Self::Three => &Self::THREE,
            Self::Four => &Self::FOUR,
            Self::Five => &Self::FIVE,
            Self::Six => &Self::SIX,
            Self::Seven => &Self::SEVEN,
            Self::Eight => &Self::EIGHT,
            Self::Nine => &Self::NINE,
        }
    }
    pub fn top(&self, rows: usize) -> &[RowRepresentation] {
        let rep = self.representation();
        &rep[..rows]
    }
    pub fn bottom(&self, rows: usize) -> &[RowRepresentation] {
        let rep = self.representation();
        &rep[rep.len() - rows..]
    }
}
impl Display for Digit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.representation() {
            for col in row {
                write!(f, "{}", col)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_from_usize() {
        assert_eq!(Digit::mod_10(0), Digit::Zero);
        assert_eq!(Digit::mod_10(1), Digit::One);
        assert_eq!(Digit::mod_10(2), Digit::Two);
        assert_eq!(Digit::mod_10(3), Digit::Three);
        assert_eq!(Digit::mod_10(4), Digit::Four);
        assert_eq!(Digit::mod_10(5), Digit::Five);
        assert_eq!(Digit::mod_10(6), Digit::Six);
        assert_eq!(Digit::mod_10(7), Digit::Seven);
        assert_eq!(Digit::mod_10(8), Digit::Eight);
        assert_eq!(Digit::mod_10(9), Digit::Nine);

        assert_eq!(Digit::mod_10(10), Digit::Zero);
        assert_eq!(Digit::mod_10(11), Digit::One);
        assert_eq!(Digit::mod_10(12), Digit::Two);
        assert_eq!(Digit::mod_10(13), Digit::Three);
        assert_eq!(Digit::mod_10(14), Digit::Four);
        assert_eq!(Digit::mod_10(15), Digit::Five);
        assert_eq!(Digit::mod_10(16), Digit::Six);
        assert_eq!(Digit::mod_10(17), Digit::Seven);
        assert_eq!(Digit::mod_10(18), Digit::Eight);
        assert_eq!(Digit::mod_10(19), Digit::Nine);
    }

    #[test]
    fn test_digit_lettering() {
        assert_eq!(
            Digit::Three.to_string(),
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
}

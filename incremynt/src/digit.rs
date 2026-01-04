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
    pub const fn increment(&self) -> Self {
        // TODO implement Digit -> usize ?
        match self {
            Self::Space => Self::Space,
            Self::Zero => Self::One,
            Self::One => Self::Two,
            Self::Two => Self::Three,
            Self::Three => Self::Four,
            Self::Four => Self::Five,
            Self::Five => Self::Six,
            Self::Six => Self::Seven,
            Self::Seven => Self::Eight,
            Self::Eight => Self::Nine,
            Self::Nine => Self::Zero,
        }
    }
    pub fn digits(mut n: usize) -> Vec<Self> {
        let mut d = Vec::new();
        while n > 0 {
            d.push(Self::mod_10(n));
            n /= 10;
        }
        d.reverse();
        d
    }
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
        self.representation().iter().try_fold((), |(), row| {
            row.iter().try_fold((), |(), col| write!(f, "{}", col))?;
            writeln!(f)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest::rstest]
    #[case(0, Digit::Zero)]
    #[case(1, Digit::One)]
    #[case(2, Digit::Two)]
    #[case(3, Digit::Three)]
    #[case(4, Digit::Four)]
    #[case(5, Digit::Five)]
    #[case(6, Digit::Six)]
    #[case(7, Digit::Seven)]
    #[case(8, Digit::Eight)]
    #[case(9, Digit::Nine)]
    #[case(10, Digit::Zero)]
    #[case(12, Digit::Two)]
    #[case(123, Digit::Three)]
    #[case(1234, Digit::Four)]
    fn test_digit_from_usize(#[case] n: usize, #[case] expected: Digit) {
        assert_eq!(Digit::mod_10(n), expected);
    }

    #[rstest::rstest]
    #[case(
        Digit::Three,
        indoc::indoc! {"
            ┏━━┓
            ┗━┓┃
            ┏━┛┃
            ┗━┓┃
            ┏━┛┃
            ┗━━┛
        "}
    )]
    #[case(
        Digit::Nine,
        indoc::indoc! {"
            ┏━━┓
            ┃┏┓┃
            ┃┗┛┃
            ┗━┓┃
            ┏━┛┃
            ┗━━┛
        "}
    )]
    fn test_digit_lettering(#[case] digit: Digit, #[case] expected: &str) {
        assert_eq!(digit.to_string(), expected);
    }
}

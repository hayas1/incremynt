#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Digit(pub [[char; 4]; 6]);
impl From<usize> for Digit {
    fn from(d: usize) -> Self {
        match d {
            0..=9 => Digit(DIGITS[d]),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Digits(pub Vec<Digit>);
impl From<usize> for Digits {
    fn from(ds: usize) -> Self {
        let mut digits = Vec::new();
        let mut curr = ds;
        while curr > 0 {
            digits.push(Digit::from(curr % 10));
            curr /= 10;
        }
        digits.reverse();
        Digits(digits)
    }
}

pub const SPACE: [[char; 4]; 6] = [
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' '],
];

pub const ZERO: [[char; 4]; 6] = [
    ['┏', '━', '━', '┓'],
    ['┃', '┏', '┓', '┃'],
    ['┃', '┃', '┃', '┃'],
    ['┃', '┃', '┃', '┃'],
    ['┃', '┗', '┛', '┃'],
    ['┗', '━', '━', '┛'],
];

pub const ONE: [[char; 4]; 6] = [
    [' ', '┏', '┓', ' '],
    [' ', '┃', '┃', ' '],
    [' ', '┃', '┃', ' '],
    [' ', '┃', '┃', ' '],
    [' ', '┃', '┃', ' '],
    [' ', '┗', '┛', ' '],
];

pub const TWO: [[char; 4]; 6] = [
    ['┏', '━', '━', '┓'],
    ['┗', '━', '┓', '┃'],
    ['┏', '━', '┛', '┃'],
    ['┃', '┏', '━', '┛'],
    ['┃', '┗', '━', '┓'],
    ['┗', '━', '━', '┛'],
];

pub const THREE: [[char; 4]; 6] = [
    ['┏', '━', '━', '┓'],
    ['┗', '━', '┓', '┃'],
    ['┏', '━', '┛', '┃'],
    ['┗', '━', '┓', '┃'],
    ['┏', '━', '┛', '┃'],
    ['┗', '━', '━', '┛'],
];

pub const FOUR: [[char; 4]; 6] = [
    ['┏', '┓', '┏', '┓'],
    ['┃', '┃', '┃', '┃'],
    ['┃', '┗', '┛', '┃'],
    ['┗', '━', '┓', '┃'],
    [' ', ' ', '┃', '┃'],
    [' ', ' ', '┗', '┛'],
];

pub const FIVE: [[char; 4]; 6] = [
    ['┏', '━', '━', '┓'],
    ['┃', '┏', '━', '┛'],
    ['┃', '┗', '━', '┓'],
    ['┗', '━', '┓', '┃'],
    ['┏', '━', '┛', '┃'],
    ['┗', '━', '━', '┛'],
];

pub const SIX: [[char; 4]; 6] = [
    ['┏', '━', '━', '┓'],
    ['┃', '┏', '━', '┛'],
    ['┃', '┗', '━', '┓'],
    ['┃', '┏', '┓', '┃'],
    ['┃', '┗', '┛', '┃'],
    ['┗', '━', '━', '┛'],
];

pub const SEVEN: [[char; 4]; 6] = [
    ['┏', '━', '━', '┓'],
    ['┃', '┏', '┓', '┃'],
    ['┗', '┛', '┃', '┃'],
    [' ', ' ', '┃', '┃'],
    [' ', ' ', '┃', '┃'],
    [' ', ' ', '┗', '┛'],
];

pub const EIGHT: [[char; 4]; 6] = [
    ['┏', '━', '━', '┓'],
    ['┃', '┏', '┓', '┃'],
    ['┃', '┗', '┛', '┃'],
    ['┃', '┏', '┓', '┃'],
    ['┃', '┗', '┛', '┃'],
    ['┗', '━', '━', '┛'],
];

pub const NINE: [[char; 4]; 6] = [
    ['┏', '━', '━', '┓'],
    ['┃', '┏', '┓', '┃'],
    ['┃', '┗', '┛', '┃'],
    ['┗', '━', '┓', '┃'],
    ['┏', '━', '┛', '┃'],
    ['┗', '━', '━', '┛'],
];

pub const DIGITS: [[[char; 4]; 6]; 10] =
    [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

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
            let mut prefix = Digits(vec![Digit(SPACE); p - self.0.len()]);
            for d in &self.0 {
                prefix.0.push(d.clone());
            }
            prefix
        } else {
            self.clone()
        }
    }
}

pub struct Incremint {
    pub prev: Digits,
    pub next: Digits,
}
impl std::fmt::Display for Incremint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..8 {
            if i < 1 {
                for (rp, rn) in self.prev.0.iter().zip(self.next.0.iter()) {
                    if rp == rn {
                        for j in 0..4 {
                            write!(f, "{}", SPACE[i + 2][j])?;
                        }
                    } else {
                        for c in rn.0[i + 2] {
                            write!(f, "{}", c)?;
                        }
                    }
                }
            } else if i < 4 {
                for (rp, rn) in self.prev.0.iter().zip(self.next.0.iter()) {
                    if rp == rn {
                        for j in 0..4 {
                            write!(f, "{}", rp.0[i - 1][j])?;
                        }
                    } else {
                        for c in rn.0[i + 2] {
                            write!(f, "{}", c)?;
                        }
                    }
                }
            } else if i < 7 {
                for (rp, rn) in self.prev.0.iter().zip(self.next.0.iter()) {
                    if rp == rn {
                        for j in 0..4 {
                            write!(f, "{}", rp.0[i - 1][j])?;
                        }
                    } else {
                        for c in rp.0[i - 4] {
                            write!(f, "{}", c)?;
                        }
                    }
                }
            } else if i < 8 {
                for (rp, rn) in self.prev.0.iter().zip(self.next.0.iter()) {
                    if rp == rn {
                        for j in 0..4 {
                            write!(f, "{}", SPACE[i - 2][j])?;
                        }
                    } else {
                        for c in rp.0[i - 4] {
                            write!(f, "{}", c)?;
                        }
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

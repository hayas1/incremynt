pub type Digit = [[char; 4]; 6];

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

pub trait Show {
    fn show<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()>;
}
impl Show for Digit {
    fn show<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        for row in self {
            for c in row {
                write!(w, "{}", c)?;
            }
            writeln!(w)?;
        }
        Ok(())
    }
}
impl Show for Vec<Digit> {
    fn show<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        for i in 0..6 {
            for row in self {
                for c in row[i] {
                    write!(w, "{}", c)?;
                }
            }
            writeln!(w)?;
        }
        Ok(())
    }
}

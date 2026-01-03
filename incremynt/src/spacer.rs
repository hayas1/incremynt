use std::fmt::Display;

use crate::Digit;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum Space {
    #[default]
    Half,
    Full,
}
impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Half => write!(f, "\u{0020}"),
            Self::Full => write!(f, "\u{3000}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Spacer<W> {
    writer: W,
    space: Space,
    scale: usize,
}
impl<W> Spacer<W> {
    pub fn new(writer: W, space: Space, scale: usize) -> Self {
        Spacer {
            writer,
            space,
            scale,
        }
    }
    pub fn scaled(&self, s: &str) -> String {
        s.replace(
            Digit::SPACE[0][0],
            &self.space.to_string().repeat(self.scale),
        )
    }
    pub fn io_write(self) -> IoSpacer<W> {
        IoSpacer(self)
    }
}
impl<W> std::fmt::Write for Spacer<W>
where
    W: std::fmt::Write,
{
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        write!(self.writer, "{}", self.scaled(s))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct IoSpacer<W>(Spacer<W>);
impl<W> std::fmt::Write for IoSpacer<W>
where
    W: std::io::Write,
{
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        write!(self.0.writer, "{}", self.0.scaled(s)).map_err(|_| std::fmt::Error)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Progress, Slot, SlotsArea};

    use super::*;

    #[rstest::rstest]
    #[case(Space::Half, " ")]
    #[case(Space::Full, "　")]
    fn test_space_width(#[case] space: Space, #[case] expected: &str) {
        assert_eq!(space.to_string(), expected);
    }

    #[rstest::rstest]
    #[case(
        Space::Full, 1,
        SlotsArea::new(
            vec![
                Slot::new(
                    Digit::Two,
                    Some(Progress::new(Digit::Three, Progress::half_progress())),
                ),
                Slot::new(Digit::Zero, None),
                Slot::new(Digit::Two, None),
                Slot::new(Digit::Four, None),
            ],
            SlotsArea::rows_hight(),
        ),
        vec![
            "┏━┛┃\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}",
            "┗━┓┃┏━━┓┏━━┓┏┓┏┓",
            "┏━┛┃┃┏┓┃┗━┓┃┃┃┃┃",
            "┗━━┛┃┃┃┃┏━┛┃┃┗┛┃",
            "┏━━┓┃┃┃┃┃┏━┛┗━┓┃",
            "┗━┓┃┃┗┛┃┃┗━┓\u{3000}\u{3000}┃┃",
            "┏━┛┃┗━━┛┗━━┛\u{3000}\u{3000}┗┛",
            "┃┏━┛\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}",
            "",
        ]
    )]
    #[case(
        Space::Half, 2,
        SlotsArea::new(
            vec![
                Slot::new(
                    Digit::Two,
                    Some(Progress::new(Digit::Three, Progress::half_progress())),
                ),
                Slot::new(Digit::Zero, None),
                Slot::new(Digit::Two, None),
                Slot::new(Digit::Four, None),
            ],
            SlotsArea::rows_hight(),
        ),
        vec![
            "┏━┛┃                        ",
            "┗━┓┃┏━━┓┏━━┓┏┓┏┓",
            "┏━┛┃┃┏┓┃┗━┓┃┃┃┃┃",
            "┗━━┛┃┃┃┃┏━┛┃┃┗┛┃",
            "┏━━┓┃┃┃┃┃┏━┛┗━┓┃",
            "┗━┓┃┃┗┛┃┃┗━┓    ┃┃",
            "┏━┛┃┗━━┛┗━━┛    ┗┛",
            "┃┏━┛                        ",
            "",
        ]
    )]
    fn test_write_spacer(
        #[case] space: Space,
        #[case] scale: usize,
        #[case] area: SlotsArea,
        #[case] expected: Vec<&str>,
    ) {
        use std::fmt::Write;
        let mut buf = String::new();
        write!(Spacer::new(&mut buf, space, scale), "{area}").unwrap();
        assert_eq!(buf, expected.join("\n"));
    }
}

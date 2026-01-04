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
pub struct Spacer {
    pub space: Space,
    pub scale: usize,
}
impl Spacer {
    pub fn new(space: Space, scale: usize) -> Self {
        Spacer { space, scale }
    }
    pub fn scaled(&self, s: &str) -> String {
        s.replace(
            Digit::SPACE[0][0],
            &self.space.to_string().repeat(self.scale),
        )
    }
    pub fn fmt_write<W>(self, writer: W) -> FmtSpacer<W> {
        FmtSpacer {
            writer,
            spacer: self,
        }
    }
    pub fn io_write<W>(self, writer: W) -> IoSpacer<W> {
        IoSpacer {
            writer,
            spacer: self,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct FmtSpacer<W> {
    writer: W,
    spacer: Spacer,
}
impl<W> FmtSpacer<W> {
    pub fn write(&self) -> &W {
        &self.writer
    }
}
impl<W> std::fmt::Write for FmtSpacer<W>
where
    W: std::fmt::Write,
{
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        write!(self.writer, "{}", self.spacer.scaled(s))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct IoSpacer<W> {
    writer: W,
    spacer: Spacer,
}
impl<W> IoSpacer<W> {
    pub fn write(&self) -> &W {
        &self.writer
    }
}
impl<W> std::fmt::Write for IoSpacer<W>
where
    W: std::io::Write,
{
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        write!(self.writer, "{}", self.spacer.scaled(s)).map_err(|_| std::fmt::Error)
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
        &[
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
        &[
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
        #[case] expected: &[&str],
    ) {
        use std::fmt::Write;
        let mut buf = String::new();
        write!(Spacer::new(space, scale).fmt_write(&mut buf), "{area}").unwrap();
        assert_eq!(buf, expected.join("\n"));
    }
}

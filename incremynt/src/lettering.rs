use std::fmt::{Display, Formatter};

use crate::{digit::Digit, ROWS};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Progress {
    next: Digit,
    progress: usize,
}
impl Progress {
    pub fn new(next: Digit, progress: usize) -> Self {
        // TODO validate
        Self { next, progress }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Slot {
    prev: Digit,
    next: Option<Progress>,
    hight: usize,
}
impl Slot {
    pub fn new(prev: Digit, next: Option<Progress>, hight: usize) -> Self {
        Self { prev, next, hight }
    }
}
impl Display for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let &Some(Progress { ref next, progress }) = &self.next {
            for bottom in next.bottom(progress) {
                for col in bottom {
                    write!(f, "{}", col)?
                }
                writeln!(f)?
            }
            for top in self.prev.top(self.hight - progress) {
                for col in top {
                    write!(f, "{}", col)?
                }
                writeln!(f)?
            }
            Ok(())
        } else {
            for space in Digit::Space.bottom((self.hight - ROWS) / 2) {
                for col in space {
                    write!(f, "{}", col)?
                }
                writeln!(f)?
            }
            write!(f, "{}", self.prev)?;
            for space in Digit::Space.top((self.hight - ROWS) / 2) {
                for col in space {
                    write!(f, "{}", col)?
                }
                writeln!(f)?
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_lettering() {
        let keep = Slot::new(Digit::Three, None, ROWS + 2);
        assert_eq!(
            keep.to_string(),
            vec![
                "    ",
                "┏━━┓",
                "┗━┓┃",
                "┏━┛┃",
                "┗━┓┃",
                "┏━┛┃",
                "┗━━┛",
                "    ",
                "",
            ]
            .join("\n")
        );

        let away = Slot::new(
            Digit::Four,
            Some(Progress::new(Digit::Five, (ROWS + 2) / 2)),
            ROWS + 2,
        );
        assert_eq!(
            away.to_string(),
            vec![
                "┃┗━┓",
                "┗━┓┃",
                "┏━┛┃",
                "┗━━┛",
                "┏┓┏┓",
                "┃┃┃┃",
                "┃┗┛┃",
                "┗━┓┃",
                "",
            ]
            .join("\n")
        );

        let progress = Slot::new(
            Digit::Four,
            Some(Progress::new(Digit::Five, (ROWS + 2) / 2 + 1)),
            ROWS + 2,
        );
        assert_eq!(
            progress.to_string(),
            vec![
                "┃┏━┛",
                "┃┗━┓",
                "┗━┓┃",
                "┏━┛┃",
                "┗━━┛",
                "┏┓┏┓",
                "┃┃┃┃",
                "┃┗┛┃",
                ""
            ]
            .join("\n")
        );
    }
}

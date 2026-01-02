use std::fmt::{Display, Formatter};

use crate::{digit::Digit, ROWS};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Slot {
    prev: Digit,
    next: Option<Digit>,
    hight: usize,
    progress: usize,
}
impl Slot {
    pub fn new(prev: Digit, next: Option<Digit>) -> Self {
        let (hight, progress) = (ROWS + 2, (ROWS + 2) / 2);
        Self::new_with(prev, next, hight, progress)
    }
    pub fn new_with(prev: Digit, next: Option<Digit>, hight: usize, progress: usize) -> Self {
        // TODO validate
        Self {
            prev,
            next,
            progress,
            hight,
        }
    }
}
impl Display for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(next) = &self.next {
            for bottom in next.bottom(self.progress) {
                for col in bottom {
                    write!(f, "{}", col)?
                }
                writeln!(f)?
            }
            for top in self.prev.top(self.hight - self.progress) {
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
        let keep = Slot::new(Digit::Three, None);
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

        let away = Slot::new(Digit::Four, Some(Digit::Five));
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
    }
}

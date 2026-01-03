use std::fmt::{Display, Formatter};

use crate::{digit::Digit, RowRepresentation, ROWS};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Progress {
    next: Digit,
    progress: usize,
}
impl Progress {
    pub fn half_progress() -> usize {
        (ROWS + 2) / 2
    }
    pub fn new(next: Digit, progress: usize) -> Self {
        // TODO validate
        Self { next, progress }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Slot {
    prev: Digit,
    next: Option<Progress>,
}
impl Slot {
    pub fn new(prev: Digit, next: Option<Progress>) -> Self {
        Self { prev, next }
    }
    pub fn row(&self, i: usize, hight: usize) -> &RowRepresentation {
        if let &Some(Progress { ref next, progress }) = &self.next {
            if i < progress {
                &next.bottom(progress)[i]
            } else if i < hight {
                &self.prev.top(hight - progress)[i - progress]
            } else {
                unreachable!()
            }
        } else {
            if i < (hight - ROWS) / 2 {
                &Digit::Space.bottom((hight - ROWS) / 2)[i]
            } else if i < (hight - ROWS) / 2 + ROWS {
                &self.prev.representation()[i - (hight - ROWS) / 2]
            } else if i < hight {
                &Digit::Space.top((hight - ROWS) / 2)[i - (hight - ROWS) / 2 - ROWS]
            } else {
                unreachable!()
            }
        }
    }
    pub fn rows(&self, hight: usize) -> Vec<&RowRepresentation> {
        (0..hight).map(|i| self.row(i, hight)).collect()
    }
}
impl Display for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.rows(SlotsArea::rows_hight())
            .iter()
            .try_fold((), |(), row| {
                row.iter().try_fold((), |(), col| write!(f, "{}", col))?;
                writeln!(f)
            })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct SlotsArea {
    slots: Vec<Slot>,
    hight: usize,
}
impl SlotsArea {
    pub fn rows_hight() -> usize {
        ROWS + 2
    }
    pub fn new(slots: Vec<Slot>, hight: usize) -> Self {
        Self { slots, hight }
    }
    pub fn rows(&self) -> Vec<Vec<&RowRepresentation>> {
        (0..self.hight)
            .map(|i| self.slots.iter().map(|s| s.row(i, self.hight)).collect())
            .collect()
    }
}
impl Display for SlotsArea {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.rows().iter().try_fold((), |(), row| {
            row.iter().try_fold((), |(), col| {
                col.iter().try_fold((), |(), col| write!(f, "{}", col))
            })?;
            writeln!(f)
        })
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

        let away = Slot::new(
            Digit::Four,
            Some(Progress::new(Digit::Five, Progress::half_progress())),
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
            Some(Progress::new(Digit::Five, Progress::half_progress() + 1)),
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

    #[test]
    fn test_slots_lettering() {
        let keep = SlotsArea::new(
            vec![
                Slot::new(Digit::Two, None),
                Slot::new(Digit::Zero, None),
                Slot::new(Digit::Two, None),
                Slot::new(Digit::Four, None),
            ],
            SlotsArea::rows_hight(),
        );
        assert_eq!(
            keep.to_string(),
            vec![
                "                ",
                "┏━━┓┏━━┓┏━━┓┏┓┏┓",
                "┗━┓┃┃┏┓┃┗━┓┃┃┃┃┃",
                "┏━┛┃┃┃┃┃┏━┛┃┃┗┛┃",
                "┃┏━┛┃┃┃┃┃┏━┛┗━┓┃",
                "┃┗━┓┃┗┛┃┃┗━┓  ┃┃",
                "┗━━┛┗━━┛┗━━┛  ┗┛",
                "                ",
                "",
            ]
            .join("\n")
        );

        let away = SlotsArea::new(
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
        );
        assert_eq!(
            away.to_string(),
            vec![
                "┏━┛┃            ",
                "┗━┓┃┏━━┓┏━━┓┏┓┏┓",
                "┏━┛┃┃┏┓┃┗━┓┃┃┃┃┃",
                "┗━━┛┃┃┃┃┏━┛┃┃┗┛┃",
                "┏━━┓┃┃┃┃┃┏━┛┗━┓┃",
                "┗━┓┃┃┗┛┃┃┗━┓  ┃┃",
                "┏━┛┃┗━━┛┗━━┛  ┗┛",
                "┃┏━┛            ",
                "",
            ]
            .join("\n")
        );
    }
}

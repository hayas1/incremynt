#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Incremint {
    pub prev: super::digit::Digits,
    pub next: super::digit::Digits,
}
impl From<(usize, usize)> for Incremint {
    fn from((prev, next): (usize, usize)) -> Self {
        Self::new(prev.into(), next.into())
    }
}
impl std::fmt::Display for Incremint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.writer(super::space::Width::Half, 1))
    }
}
impl Incremint {
    pub fn new(mut prev: super::digit::Digits, mut next: super::digit::Digits) -> Self {
        let len = prev.len().max(next.len());
        prev.padding(super::digit::Digit::ZERO, len);
        next.padding(super::digit::Digit::ZERO, len);
        Self { prev, next }
    }
    pub fn writer(
        &self,
        space: super::space::Width,
        scale: usize,
    ) -> super::write::IncremintWriter {
        super::write::IncremintWriter::new(self, space, scale)
    }
}

#[cfg(test)]
mod tests {
    use crate::digit::Digits;

    use super::*;

    #[test]
    fn test_incremint_display() {
        let incremint = Incremint::new(Digits::from(2024), Digits::from(3024));
        assert_eq!(
            incremint.to_string(),
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

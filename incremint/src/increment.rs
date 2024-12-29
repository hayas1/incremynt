#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Incremint {
    pub prev: super::digit::Digits,
    pub next: super::digit::Digits,
}
impl std::fmt::Display for Incremint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.writer(1))
    }
}
impl Incremint {
    pub fn writer(&self, scale: usize) -> super::write::IncremintWriter {
        super::write::IncremintWriter::new(self, scale)
    }
}

#[cfg(test)]
mod tests {
    use crate::digit::Digits;

    use super::*;

    #[test]
    fn test_incremint_display() {
        let incremint = Incremint {
            prev: Digits::from(2024),
            next: Digits::from(3024),
        };
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

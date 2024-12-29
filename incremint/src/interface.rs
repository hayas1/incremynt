#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Application<D> {
    pub d: D,
    pub space: super::space::Width,
    pub scale: usize,
}
impl Application<crate::digit::Digit> {
    pub fn run<W: std::io::Write>(self, w: &mut W) -> std::io::Result<()> {
        write!(w, "{}", self.d.writer(self.space.into(), self.scale))
    }
}
impl Application<crate::digit::Digits> {
    pub fn run<W: std::io::Write>(self, w: &mut W) -> std::io::Result<()> {
        write!(w, "{}", self.d.writer(self.space.into(), self.scale))
    }
}
impl Application<crate::increment::Incremint> {
    pub fn run<W: std::io::Write>(self, w: &mut W) -> std::io::Result<()> {
        write!(w, "{}", self.d.writer(self.space.into(), self.scale))
    }
}

#[cfg(test)]
mod tests {
    use crate::{increment::Incremint, space::Width};

    use super::*;

    #[test]
    fn test_basic_application() {
        let program = Application::<Incremint> {
            d: Incremint::new(2024.into(), 3024.into()),
            space: Width::Full,
            scale: 1,
        };

        let mut buffer = Vec::new();
        program.run(&mut buffer).unwrap();
        assert_eq!(
            String::from_utf8_lossy(&buffer),
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
            .join("\n")
        );
    }
}

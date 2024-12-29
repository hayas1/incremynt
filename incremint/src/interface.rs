#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Application {
    pub prev: usize,
    pub next: usize,
    pub space: super::space::Width,
    pub scale: usize,
}
impl Application {
    pub fn run<W: std::io::Write>(self, w: &mut W) -> std::io::Result<()> {
        let (prev, next) = (self.prev.into(), self.next.into());
        write!(
            w,
            "{}",
            super::increment::Incremint::new(prev, next).writer(self.space.into(), self.scale)
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::space::Width;

    use super::*;

    #[test]
    fn test_basic_application() {
        let program = Application {
            prev: 2024,
            next: 3024,
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

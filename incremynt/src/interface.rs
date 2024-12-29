#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Application<D> {
    pub d: D,
    pub space: super::space::Width,
    pub scale: usize,
}
impl<D> Application<D> {
    pub fn run<'a, W>(&'a self, w: &mut W) -> std::io::Result<()>
    where
        D: super::write::Writable<'a>,
        D::Writer: std::fmt::Display,
        W: std::io::Write,
    {
        let Self { d, space, scale } = self;
        let writer = d.writer(space.clone(), *scale);
        write!(w, "{}", writer)
    }
}

#[cfg(test)]
mod tests {
    use crate::{increment::Incremynt, space::Width};

    use super::*;

    #[test]
    fn test_basic_application() {
        let program = Application::<Incremynt> {
            d: Incremynt::new(2024.into(), 3024.into()),
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

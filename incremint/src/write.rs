#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Writer<D> {
    d: D,
    scale: usize,
}
impl<D> Writer<D> {
    pub fn new(d: D, scale: usize) -> Self {
        Self { d, scale }
    }
    pub fn space_scaled(&self, c: char) -> impl Iterator<Item = char> {
        let rep = if c == crate::SPACE[0][0] {
            self.scale
        } else {
            1
        };
        (0..rep).map(move |_| c)
    }
    pub fn digit_row<'a>(
        &'a self,
        digit: &'a super::digit::Digit,
        row: usize,
    ) -> impl 'a + Iterator<Item = char> {
        let r = &digit[row];
        r.iter().flat_map(move |&c| self.space_scaled(c))
    }
}

// impl<D> std::fmt::Display for Writer<&D>
// where
//     Writer<D>: std::fmt::Display,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self)
//     }
// }
// impl<D> std::fmt::Display for Writer<&mut D>
// where
//     Writer<D>: std::fmt::Display,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self)
//     }
// }
impl std::fmt::Display for Writer<super::digit::Digit> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..crate::ROWS {
            for x in self.digit_row(&self.d, row) {
                write!(f, "{}", x)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Writer<super::digit::Digits> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..crate::ROWS {
            for digit in self.d.iter() {
                for x in self.digit_row(digit, row) {
                    write!(f, "{}", x)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Writer<super::increment::Incremint> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..8 {
            if row < 1 {
                for (dp, dn) in self.d.prev.iter().zip(self.d.next.iter()) {
                    let d = if dp == dn {
                        &super::digit::Digit::SPACE
                    } else {
                        dn
                    };
                    for x in self.digit_row(d, row + 2) {
                        write!(f, "{}", x)?;
                    }
                }
            } else if row < 4 {
                for (dp, dn) in self.d.prev.iter().zip(self.d.next.iter()) {
                    let r = if dp == dn { row - 1 } else { row + 2 };
                    for x in self.digit_row(dn, r) {
                        write!(f, "{}", x)?;
                    }
                }
            } else if row < 7 {
                for (dp, dn) in self.d.prev.iter().zip(self.d.next.iter()) {
                    let r = if dp == dn { row - 1 } else { row - 4 };
                    for x in self.digit_row(dp, r) {
                        write!(f, "{}", x)?;
                    }
                }
            } else if row < 8 {
                for (dp, dn) in self.d.prev.iter().zip(self.d.next.iter()) {
                    let d = if dp == dn {
                        &super::digit::Digit::SPACE
                    } else {
                        dp
                    };
                    for x in self.digit_row(d, row - 4) {
                        write!(f, "{}", x)?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

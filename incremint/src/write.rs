pub struct Writer<D> {
    d: D,
    scale: usize,
}
impl<D> Writer<D> {
    pub fn new(d: D, scale: usize) -> Self {
        Self { d, scale }
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
        for (i, row) in self.d.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if c == &super::SPACE[i][j] {
                    for _ in 0..self.scale {
                        write!(f, "{}", c)?;
                    }
                } else {
                    write!(f, "{}", c)?;
                }
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for Writer<super::digit::Digits> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in self.d.iter() {
            for i in 0..6 {
                for (j, c) in digit[i].iter().enumerate() {
                    if c == &super::SPACE[i][j] {
                        for _ in 0..self.scale {
                            write!(f, "{}", c)?;
                        }
                    } else {
                        write!(f, "{}", c)?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Writer<super::increment::Incremint> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..8 {
            if i < 1 {
                for (rp, rn) in self.d.prev.iter().zip(self.d.next.iter()) {
                    if rp == rn {
                        for j in 0..4 {
                            write!(f, "{}", super::SPACE[i + 2][j])?;
                        }
                    } else {
                        for c in rn[i + 2] {
                            write!(f, "{}", c)?;
                        }
                    }
                }
            } else if i < 4 {
                for (rp, rn) in self.d.prev.iter().zip(self.d.next.iter()) {
                    if rp == rn {
                        for j in 0..4 {
                            write!(f, "{}", rp[i - 1][j])?;
                        }
                    } else {
                        for c in rn[i + 2] {
                            write!(f, "{}", c)?;
                        }
                    }
                }
            } else if i < 7 {
                for (rp, rn) in self.d.prev.iter().zip(self.d.next.iter()) {
                    if rp == rn {
                        for j in 0..4 {
                            write!(f, "{}", rp[i - 1][j])?;
                        }
                    } else {
                        for c in rp[i - 4] {
                            write!(f, "{}", c)?;
                        }
                    }
                }
            } else if i < 8 {
                for (rp, rn) in self.d.prev.iter().zip(self.d.next.iter()) {
                    if rp == rn {
                        for j in 0..4 {
                            write!(f, "{}", super::SPACE[i - 2][j])?;
                        }
                    } else {
                        for c in rp[i - 4] {
                            write!(f, "{}", c)?;
                        }
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

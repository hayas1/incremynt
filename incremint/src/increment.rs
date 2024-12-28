#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Incremint {
    pub prev: super::digit::Digits,
    pub next: super::digit::Digits,
}
impl std::fmt::Display for Incremint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..8 {
            if i < 1 {
                for (rp, rn) in self.prev.0.iter().zip(self.next.0.iter()) {
                    if rp == rn {
                        for j in 0..4 {
                            write!(f, "{}", super::SPACE[i + 2][j])?;
                        }
                    } else {
                        for c in rn.0[i + 2] {
                            write!(f, "{}", c)?;
                        }
                    }
                }
            } else if i < 4 {
                for (rp, rn) in self.prev.0.iter().zip(self.next.0.iter()) {
                    if rp == rn {
                        for j in 0..4 {
                            write!(f, "{}", rp.0[i - 1][j])?;
                        }
                    } else {
                        for c in rn.0[i + 2] {
                            write!(f, "{}", c)?;
                        }
                    }
                }
            } else if i < 7 {
                for (rp, rn) in self.prev.0.iter().zip(self.next.0.iter()) {
                    if rp == rn {
                        for j in 0..4 {
                            write!(f, "{}", rp.0[i - 1][j])?;
                        }
                    } else {
                        for c in rp.0[i - 4] {
                            write!(f, "{}", c)?;
                        }
                    }
                }
            } else if i < 8 {
                for (rp, rn) in self.prev.0.iter().zip(self.next.0.iter()) {
                    if rp == rn {
                        for j in 0..4 {
                            write!(f, "{}", super::SPACE[i - 2][j])?;
                        }
                    } else {
                        for c in rp.0[i - 4] {
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

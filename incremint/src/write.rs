#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct DigitsWriter<D> {
    d: D,
    scale: usize,
}
impl<D> DigitsWriter<D> {
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
impl std::fmt::Display for DigitsWriter<super::digit::Digit> {
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
impl std::fmt::Display for DigitsWriter<super::digit::Digits> {
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
// impl<D> std::fmt::Display for DigitsWriter<&D>
// where
//     DigitsWriter<D>: std::fmt::Display,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self)
//     }
// }
// impl<D> std::fmt::Display for DigitsWriter<&mut D>
// where
//     DigitsWriter<D>: std::fmt::Display,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self)
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IncremintWriter<'a> {
    inner: DigitsWriter<&'a super::increment::Incremint>,
}
impl<'a> IncremintWriter<'a> {
    pub fn new(d: &'a super::increment::Incremint, scale: usize) -> Self {
        Self {
            inner: DigitsWriter::new(d, scale),
        }
    }
    pub fn write_chunk(
        &'a self,
        (prev, next): (&'a super::digit::Digit, &'a super::digit::Digit),
        row: usize,
    ) -> (&'a super::digit::Digit, usize) {
        if row < 1 {
            let d = if prev == next {
                &super::digit::Digit::SPACE
            } else {
                next
            };
            (d, row + 2)
        } else if row < 4 {
            let r = if prev == next { row - 1 } else { row + 2 };
            (next, r)
        } else if row < 7 {
            let r = if prev == next {
                row - 1
            } else {
                row + 2 - crate::ROWS
            };
            (prev, r)
        } else if row < 8 {
            let d = if prev == next {
                &super::digit::Digit::SPACE
            } else {
                prev
            };
            (d, row + 2 - crate::ROWS)
        } else {
            unreachable!();
        }
    }
}
impl<'a> std::fmt::Display for IncremintWriter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = &self.inner;
        for row in 0..(crate::ROWS + 2) {
            for dpn in inner.d.prev.iter().zip(inner.d.next.iter()) {
                let (d, r) = self.write_chunk(dpn, row);
                for x in inner.digit_row(d, r) {
                    write!(f, "{}", x)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

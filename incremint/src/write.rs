#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DigitsWriter<'a, D> {
    d: &'a D,
    space_width: super::space::Width,
    scale: usize,
}
impl<'a, D> DigitsWriter<'a, D> {
    pub fn new(d: &'a D, space_width: super::space::Width, scale: usize) -> Self {
        Self {
            d,
            space_width,
            scale,
        }
    }
    pub fn space_scaled(&self, c: char) -> impl Iterator<Item = char> {
        let (rep, s) = if c == crate::SPACE[0][0] {
            (self.scale, self.space_width.space())
        } else {
            (1, c)
        };
        (0..rep).map(move |_| s)
    }
    pub fn digit_row(
        &'a self,
        digit: &'a super::digit::Digit,
        row: usize,
    ) -> impl 'a + Iterator<Item = char> {
        let r = &digit[row];
        r.iter().flat_map(move |&c| self.space_scaled(c))
    }
}
impl<'a> std::fmt::Display for DigitsWriter<'a, super::digit::Digit> {
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
impl<'a> std::fmt::Display for DigitsWriter<'a, super::digit::Digits> {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IncremintWriter<'a> {
    inner: DigitsWriter<'a, super::increment::Incremint>,
}
impl<'a> IncremintWriter<'a> {
    pub fn new(
        d: &'a super::increment::Incremint,
        space_width: super::space::Width,
        scale: usize,
    ) -> Self {
        Self {
            inner: DigitsWriter::new(d, space_width, scale),
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

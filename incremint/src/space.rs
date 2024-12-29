#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Width {
    Half,
    Full,
}
impl std::fmt::Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.space())
    }
}
impl Width {
    pub fn space(&self) -> char {
        match self {
            Width::Half => '\u{0020}',
            Width::Full => '\u{3000}',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_width() {
        assert_eq!(Width::Half.space(), ' ');
        assert_eq!(Width::Full.space(), '　');
    }
}

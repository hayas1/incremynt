#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Incremint {
    pub prev: super::digit::Digits,
    pub next: super::digit::Digits,
}
impl std::fmt::Display for Incremint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let writer = super::write::Writer::new(self.clone(), 1);
        write!(f, "{}", writer)
    }
}

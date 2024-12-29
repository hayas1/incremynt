#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Width {
    Half,
    Full,
}
impl std::fmt::Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Width::Half => write!(f, "half"),
            Width::Full => write!(f, "full"),
        }
    }
}
impl Width {
    pub fn space(&self) -> char {
        match self {
            Width::Half => ' ',
            Width::Full => '　',
        }
    }
}

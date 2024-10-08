pub(crate) enum Classification {
    STANDARD, SPECIAL, REJECTED
}

impl std::fmt::Display for Classification {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Classification::STANDARD => write!(f, "standard"),
            Classification::SPECIAL => write!(f, "special"),
            Classification::REJECTED => write!(f, "rejected")
        }
    }
}
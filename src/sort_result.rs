use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SortResult {
    Standard, Special, Rejected
}

#[cfg(test)]
impl SortResult {
    pub(crate) fn any() -> [Self; 3] {
        [SortResult::Standard, SortResult::Special, SortResult::Rejected]
    }
}

impl std::fmt::Display for SortResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortResult::Standard => write!(f, "standard"),
            SortResult::Special => write!(f, "special"),
            SortResult::Rejected => write!(f, "rejected")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any() {
        assert_eq!(SortResult::any(), [SortResult::Standard, SortResult::Special, SortResult::Rejected]);
    }

    #[test]
    fn display_standard() {
        assert_eq!(SortResult::Standard.to_string(), "standard");
    }

    #[test]
    fn display_special() {
        assert_eq!(SortResult::Special.to_string(), "special");
    }

    #[test]
    fn display_rejected() {
        assert_eq!(SortResult::Rejected.to_string(), "rejected");
    }
}
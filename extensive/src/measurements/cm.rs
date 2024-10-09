use std::cmp::Ordering;

pub const MIN: usize = 1;
pub const MAX: usize = usize::MAX;

#[derive(Clone, Debug)]
pub(crate) struct Cm {
    value: usize
}

#[derive(Clone, Debug)]
pub(crate) enum CmError {
    InvalidCm(usize)
}

impl CmError {
    pub(crate) fn original_value(&self) -> usize {
        match self {
            CmError::InvalidCm(value) => value.clone()
        }
    }
}

impl Cm {
    fn is_valid(value: usize) -> bool {
        value >= 1
    }

    pub(crate) fn new(value: usize) -> Result<Self, CmError> {
        if !Cm::is_valid(value) {
            return Err(
                CmError::InvalidCm(value)
            )
        }

        Ok(
            Cm {
                value
            }
        )
    }
}

impl std::ops::Add for Cm {
    type Output = Cm;

    fn add(self, other: Cm) -> Cm {
        Cm::new(self.value.saturating_add(other.value)).unwrap()
    }
}

impl PartialEq for Cm {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Cm {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::Arbitrary;
    use rand::Rng;
    use super::*;

    impl Arbitrary for Cm {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let value = rand::thread_rng().gen_range(MIN..=MAX);
            Cm::new(value).unwrap()
        }
    }

    #[quickcheck]
    fn values_are_equal_to_themselves(value: Cm) {
        assert_eq!(value, value)
    }

    #[quickcheck]
    fn adding_a_positive_value_increases_value(value: Cm, other: Cm) {
        let result = value.clone() + other.clone();
        assert_ne!(value, result)
    }
}
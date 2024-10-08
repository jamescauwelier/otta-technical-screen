#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub(crate) struct Kg {
    value: usize
}

#[derive(Clone, Debug)]
pub(crate) enum KgError {
    InvalidKg(usize)
}

impl Kg {
    fn is_valid(value: usize) -> bool {
        value >= 1
    }

    pub(crate) fn new(value: usize) -> Result<Kg, KgError> {
        if !Kg::is_valid(value) {
            return Err(
                KgError::InvalidKg(value)
            )
        }

        Ok(
            Kg {
                value
            }
        )
    }
}
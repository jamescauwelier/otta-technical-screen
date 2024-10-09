use crate::measurements::kg::KgError;

/**
 * SortError
 *
 * Documents the potential error states that can occur when
 * sorting a package. At this point, all of these are input errors.
 */
#[derive(Clone, Debug, PartialEq)]
pub enum SortError {
    InvalidHeight(usize),
    InvalidWidth(usize),
    InvalidLength(usize),
    InvalidMass(usize)
}

impl std::fmt::Display for SortError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SortError::InvalidHeight(height) => write!(f, "Invalid height: expecting a value of 1 or more, but got {}", height),
            SortError::InvalidWidth(width) => write!(f, "Invalid width: expecting a value of 1 or more, but got {}", width),
            SortError::InvalidLength(length) => write!(f, "Invalid length: expecting a value of 1 or more, but got {}", length),
            SortError::InvalidMass(mass) => write!(f, "Invalid mass: expecting a value of 1 or more, but got {}", mass)
        }
    }
}

impl From<KgError> for SortError {
    fn from(error: KgError) -> Self {
        match error {
            KgError::InvalidKg(value) => SortError::InvalidMass(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn display_invalid_height(height: usize) {
        assert_eq!(SortError::InvalidHeight(height).to_string(), format!("Invalid height: expecting a value of 1 or more, but got {}", height));
    }

    #[quickcheck]
    fn display_invalid_width(width: usize) {
        assert_eq!(SortError::InvalidWidth(width).to_string(), format!("Invalid width: expecting a value of 1 or more, but got {}", width));
    }

    #[quickcheck]
    fn display_invalid_length(length: usize) {
        assert_eq!(SortError::InvalidLength(length).to_string(), format!("Invalid length: expecting a value of 1 or more, but got {}", length));
    }

    #[quickcheck]
    fn display_invalid_mass(mass: usize) {
        assert_eq!(SortError::InvalidMass(mass).to_string(), format!("Invalid mass: expecting a value of 1 or more, but got {}", mass));
    }
}
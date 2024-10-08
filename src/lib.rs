#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use crate::measurements::cm::Cm;
use crate::measurements::kg::Kg;
use crate::package::dimensions::PackageDimensions;
use crate::package::mass::Mass;
use crate::package::Package;
use crate::sort_error::SortError;
use crate::sort_result::SortResult;

pub(crate) mod measurements;
mod package;
mod sort_result;
pub(crate)  mod sort_error;

/**
 * Safe sort
 *
 * This sort function should not fail, but doesn't honor the
 * original specification.
 */
fn safe_sort(width: usize, height: usize, length: usize, mass: usize) -> Result<SortResult, SortError> {

    // type conversions to impose domain invariants
    let width_cm = Cm::new(width).map_err(|e| SortError::InvalidWidth(e.original_value()))?;
    let height_cm = Cm::new(height).map_err(|e| SortError::InvalidHeight(e.original_value()))?;
    let length_cm = Cm::new(length).map_err(|e| SortError::InvalidLength(e.original_value()))?;
    let mass_kg = Kg::new(mass)?;

    // composes the Package aggregate root
    let p = Package::new(
        PackageDimensions::new(width_cm, height_cm, length_cm),
        Mass::new(mass_kg)
    );

    // performs the actual sorting
    Ok(p.sort())
}

/**
 * Safe sort 2
 *
 * Honors the original signature, but returns errors as strings
 * and doesn't allow knowing whether an error has occured
 */
#[allow(dead_code)]
fn safe_sort_2(_width: usize, _height: usize, _length: usize, _mass: usize) -> String {
    match safe_sort(_width, _height, _length, _mass) {
        Ok(result) => result.to_string(),
        Err(_) => "error".to_string()
    }
}

/**
 * Sort
 *
 * This function honors the original signature and specs, but
 * may panic if an error condition occurred. Another
 * potential solution is to reject all inputs that are invalid, but
 * it doesn't sufficiently inform the caller of what happened.
 *
 * In my opinion, the original signature needs to be improved
 * upon, especially in Rust. Other languages may option to
 * return a null value, although that can be tricky as well.
 */
#[allow(dead_code)]
fn sort(width: usize, height: usize, length: usize, mass: usize) -> String {
    safe_sort(width, height, length, mass).unwrap().to_string()
}

#[cfg(test)]
pub(crate) mod test_dependencies {
    use crate::sort_error::SortError;
    use crate::sort_result::SortResult;
    use rand::Rng;

    fn generate_within_range(min: usize, max: usize) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(min..=max)
    }

    fn generate_outside_of_range(min: usize, max: usize) -> usize {
        let generate_right = || rand::thread_rng().gen_range(max+1..=usize::MAX);
        let generate_left = || rand::thread_rng().gen_range(usize::MIN..min);
        match (
            min == usize::MIN,
            max == usize::MAX
            ) {
            (true, true) => panic!("Cannot generate a value outside of the range of a usize"),
            (true, false) => generate_right(),
            (false, true) => generate_left(),
            (false, false) => {
                match rand::thread_rng().gen_bool(0.5) {
                    true => generate_right(),
                    false => generate_left()
                }
            }
        }
    }

    /**
     * Wrapper function to main sort function
     *
     * Allows for the use of arbitrary wrapper types that need
    * unwrapping with Into<> for the main sort function to
    * accept them
     */
    pub(crate) fn sort_helper<T,U,V,W>(width: T, height: U, length: V, mass: W) -> Result<SortResult, SortError>
    where
        T: Into<usize>,
        U: Into<usize>,
        V: Into<usize>,
        W: Into<usize>
    {
        super::safe_sort(width.into(), height.into(), length.into(), mass.into())
    }

    pub(crate) mod length {
        use crate::test_dependencies::{generate_outside_of_range, generate_within_range};
        use quickcheck::Arbitrary;

        #[derive(Clone, Debug)]
        pub(crate) struct InvalidLength { value: usize }

        // the length of a dimension is always a positive integer and cannot be zero
        impl Arbitrary for InvalidLength {
            fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
                InvalidLength {
                    value: generate_outside_of_range(1, usize::MAX)
                }
            }
        }

        impl Into<usize> for InvalidLength {
            fn into(self) -> usize {
                self.value
            }
        }

        #[derive(Clone, Debug)]
        pub(crate) struct ValidLength { value: usize }

        // the length of a dimension is always a positive integer and cannot be zero
        impl Arbitrary for ValidLength {
            fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
                ValidLength {
                    value: generate_within_range(1, usize::MAX)
                }
            }
        }

        impl Into<usize> for ValidLength {
            fn into(self) -> usize {
                self.value
            }
        }
    }

    // utility functions for generating random values of mass
    // even thought his looks identical to the length module,
    // mass is a separate concept and is therefore kept separate
    // this is not considered code duplication as the concepts of
    // what constitutes a valid mass and a valid length would
    // likely evolve independently
    pub(crate) mod mass {
        use crate::test_dependencies::{generate_outside_of_range, generate_within_range};
        use quickcheck::Arbitrary;

        #[derive(Clone, Debug)]
        pub(crate) struct InvalidMass { value: usize }

        // the mass of a package is always a positive integer, and cannot be zero
        impl Arbitrary for InvalidMass {
            fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
                InvalidMass {
                    value: generate_outside_of_range(1, usize::MAX)
                }
            }
        }

        impl Into<usize> for InvalidMass {
            fn into(self) -> usize {
                self.value
            }
        }

        #[derive(Clone, Debug)]
        pub(crate) struct ValidMass { value: usize }

        // the mass of a package is always a positive integer, and cannot be zero
        impl Arbitrary for ValidMass {
            fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
                ValidMass {
                    value: generate_within_range(1, usize::MAX)
                }
            }
        }

        impl Into<usize> for ValidMass {
            fn into(self) -> usize {
                self.value
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort_error::SortError;
    use crate::sort_result::SortResult;
    use crate::test_dependencies::length::{InvalidLength, ValidLength};
    use crate::test_dependencies::mass::{InvalidMass, ValidMass};
    use crate::test_dependencies::sort_helper;

    #[quickcheck]
    fn sorting_packages_returns_one_of_4_strings(width: ValidLength, height: ValidLength, length: ValidLength, mass: ValidMass) {
        let requirement = SortResult::any()
            .contains(&sort_helper(width, height, length, mass).unwrap());
        assert!(requirement);
    }

    #[quickcheck]
    fn sorting_with_an_invalid_width_produces_an_error(width: InvalidLength, height: ValidLength, length: ValidLength, mass: ValidMass) {
        let expected = Err(SortError::InvalidWidth(width.clone().into()));
        let got = sort_helper(width, height, length, mass);
         assert_eq!(got, expected)
    }

    #[quickcheck]
    fn sorting_with_an_invalid_height_produces_an_error(width: ValidLength, height: InvalidLength, length: ValidLength, mass: ValidMass) {
        let expected = Err(SortError::InvalidHeight(height.clone().into()));
        let got = sort_helper(width, height, length, mass);
        assert_eq!(got, expected)
    }

    #[quickcheck]
    fn sorting_with_an_invalid_length_produces_an_error(width: ValidLength, height: ValidLength, length: InvalidLength, mass: ValidMass) {
        let expected = Err(SortError::InvalidLength(length.clone().into()));
        let got = sort_helper(width, height, length, mass);
        assert_eq!(got, expected)
    }

    #[quickcheck]
    fn sorting_with_an_invalid_mass_produces_an_error(width: ValidLength, height: ValidLength, length: ValidLength, mass: InvalidMass) {
        let expected = Err(SortError::InvalidMass(mass.clone().into()));
        let got = sort_helper(width, height, length, mass);
        assert_eq!(got, expected)
    }
}
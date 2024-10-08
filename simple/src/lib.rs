mod mass_class;
mod dimension_class;
mod classification;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use classification::Classification;
use dimension_class::DimensionClass;
use mass_class::MassClass;


/**
 * Sort
 *
 * Sorts packages based on dimension and mass classification
 * Errors are handled by returning an empty string.
 * The implementation handles integer overflows gracefully.
 */
fn sort(width: usize, height: usize, length: usize, mass: usize) -> String {
    if !validate_input(width, height, length, mass) {
        return "".to_string()
    }

    let classification_pair = (
        DimensionClass::from(width, height, length),
        MassClass::from(mass)
    );
    let classification_result = match classification_pair {
        (DimensionClass::STANDARD, MassClass::STANDARD) => Classification::STANDARD,
        (DimensionClass::BULKY, MassClass::STANDARD) => Classification::SPECIAL,
        (DimensionClass::STANDARD, MassClass::HEAVY) => Classification::SPECIAL,
        (DimensionClass::BULKY, MassClass::HEAVY) => Classification::REJECTED
    };

    classification_result.to_string()
}

fn validate_input(width: usize, height: usize, length: usize, mass: usize) -> bool {
    width > 0 && height > 0 && length > 0 && mass > 0
}

#[cfg(test)]
mod test_dependencies {
    use quickcheck::Arbitrary;
    use rand::Rng;

    #[derive(Clone, Debug)]
    pub(crate) struct ValidInput { value: usize }

    // valid input is anything above 0
    impl Arbitrary for ValidInput {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            ValidInput {
                value: rand::thread_rng().gen_range(1..=usize::MAX)
            }
        }
    }

    impl From<ValidInput> for usize {
        fn from(input: ValidInput) -> Self {
            input.value
        }
    }

    #[derive(Clone, Debug)]
    pub(crate) struct InvalidInput { value: usize }

    // the only invalid input is zero
    impl Arbitrary for InvalidInput {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            InvalidInput { value: 0 }
        }
    }

    impl From<InvalidInput> for usize {
        fn from(input: InvalidInput) -> Self {
            input.value
        }
    }
}

#[cfg(test)]
mod tests {
    fn expect_sort_output<T,U,V,W,X>(width: T, height: U, length: V, mass: W, expected_output: X)
    where
        T: Into<usize>,
        U: Into<usize>,
        V: Into<usize>,
        W: Into<usize>,
        X: Into<String>
    {
        assert_eq!(sort(width.into(), height.into(), length.into(), mass.into()), expected_output.into())
    }

    use crate::sort;
    use crate::test_dependencies::{InvalidInput, ValidInput};

    #[quickcheck]
    fn sorting_with_invalid_width_returns_empty_classification(width: InvalidInput, height: ValidInput, length: ValidInput, mass: ValidInput) {
        expect_sort_output(width, height, length, mass, "".to_string())
    }

    #[quickcheck]
    fn sorting_with_invalid_height_returns_empty_classification(width: ValidInput, height: InvalidInput, length: ValidInput, mass: ValidInput) {
        expect_sort_output(width, height, length, mass, "".to_string())
    }

    #[quickcheck]
    fn sorting_with_invalid_length_returns_empty_classification(width: ValidInput, height: ValidInput, length: InvalidInput, mass: ValidInput) {
        expect_sort_output(width, height, length, mass, "".to_string())
    }

    #[quickcheck]
    fn sorting_with_invalid_mass_returns_empty_classification(width: ValidInput, height: ValidInput, length: ValidInput, mass: InvalidInput) {
        expect_sort_output(width, height, length, mass, "".to_string())
    }

    #[test]
    fn test_standard_classification() {
        expect_sort_output(1_usize, 1_usize, 1_usize, 1_usize, "standard")
    }
    
    #[test]
    fn test_bulky_classification() {
        expect_sort_output(148_usize, 1_usize, 1_usize, 1_usize, "special")
    }
    
    #[test]
    fn test_heavy_classification() {
        expect_sort_output(1_usize, 1_usize, 1_usize, 20_usize, "special")
    }

    #[test]
    fn test_bulky_and_heavy_classification() {
        expect_sort_output(148_usize, 1_usize, 1_usize, 20_usize, "rejected")
    }
}
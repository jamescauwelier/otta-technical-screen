use crate::measurements::cm::Cm;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum DimensionClass {
    STANDARD, BULKY
}

#[derive(Clone, Debug)]
pub(crate) struct PackageDimensions {
    width: Cm,
    height: Cm,
    length: Cm
}

impl PackageDimensions {
    pub(crate) fn new(width: Cm, height: Cm, length: Cm) -> Self {
        PackageDimensions {
            width,
            height,
            length
        }
    }

    fn sum(&self) -> Cm {
        self.width.clone() + self.height.clone() + self.length.clone()
    }

    pub(crate) fn classify(&self) -> DimensionClass {
        if self.sum() >= Cm::new(150).unwrap() {
            DimensionClass::BULKY
        } else {
            DimensionClass::STANDARD
        }
    }
}

#[cfg(test)]
pub(crate) mod test_dependencies {
    use crate::measurements::cm::Cm;
    use crate::package::dimensions::PackageDimensions;
    use quickcheck::{Arbitrary, Gen};
    use rand::Rng;
    use std::ops::Deref;

    #[derive(Clone, Debug)]
    pub(crate) struct StandardPackageDimensions {
        dimensions: PackageDimensions
    }

    impl Deref for StandardPackageDimensions {
        type Target = PackageDimensions;

        fn deref(&self) -> &Self::Target {
            &self.dimensions
        }
    }

    impl From<StandardPackageDimensions> for PackageDimensions {
        fn from(dimensions: StandardPackageDimensions) -> Self {
            dimensions.dimensions
        }
    }

    impl Arbitrary for StandardPackageDimensions {
        fn arbitrary(_g: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();

            // we can't have a dimension of zero
            let min = 1;
            // the sum of all dimensions must be less than 150
            let max_total_dimension = 150;

            let width = rng.gen_range(min..max_total_dimension - 2);
            let height = rng.gen_range(min..(max_total_dimension - width - 1));
            let length = rng.gen_range(min..(max_total_dimension - width - height));

            StandardPackageDimensions {
                dimensions: PackageDimensions::new(
                    Cm::new(width).unwrap(),
                    Cm::new(height).unwrap(),
                    Cm::new(length).unwrap()
                )
            }
        }
    }

    #[derive(Clone, Debug)]
    pub(crate) struct BulkyPackageDimensions {
        dimensions: PackageDimensions
    }

    impl From<BulkyPackageDimensions> for PackageDimensions {
        fn from(dimensions: BulkyPackageDimensions) -> Self {
            dimensions.dimensions
        }
    }

    impl Deref for BulkyPackageDimensions {
        type Target = PackageDimensions;

        fn deref(&self) -> &Self::Target {
            &self.dimensions
        }
    }

    impl Arbitrary for BulkyPackageDimensions {
        fn arbitrary(_g: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();

            // we can't have a dimension of zero
            let mut min = 1;

            let width = rng.gen_range(min..=usize::MAX);
            let height = rng.gen_range(min..=usize::MAX);

            min = (150 as usize).saturating_sub(width).saturating_sub(height);
            let length = rng.gen_range(min..=usize::MAX);

            BulkyPackageDimensions {
                dimensions: PackageDimensions::new(
                    Cm::new(width).unwrap(),
                    Cm::new(height).unwrap(),
                    Cm::new(length).unwrap()
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::dimensions::test_dependencies::{BulkyPackageDimensions, StandardPackageDimensions};


    #[quickcheck]
    fn the_sum_of_dimensions_for_arbitrary_standard_packages_is_less_than_150_cm(dimensions: StandardPackageDimensions) -> bool {
        dimensions.sum() < Cm::new(150).unwrap()
    }

    #[quickcheck]
    fn the_sum_of_dimensions_for_arbitrary_bulky_packages_is_greater_than_or_equal_to_150_cm(dimensions: BulkyPackageDimensions) -> bool {
        dimensions.sum() >= Cm::new(150).unwrap()
    }

    #[quickcheck]
    fn a_standard_package_is_not_bulky(dimensions: StandardPackageDimensions) -> bool {
        dimensions.classify() == DimensionClass::STANDARD
    }

    #[quickcheck]
    fn a_bulky_package_is_bulky(dimensions: BulkyPackageDimensions) -> bool {
        dimensions.classify() == DimensionClass::BULKY
    }
}
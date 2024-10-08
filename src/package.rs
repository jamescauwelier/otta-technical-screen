use crate::package::dimensions::DimensionClass;
use crate::package::mass::MassClass;
use crate::sort_result::SortResult;

pub(crate) mod dimensions;
pub(crate) mod mass;

pub(crate) struct Package {
    dimensions: dimensions::PackageDimensions,
    mass: mass::Mass
}

impl Package {

    pub(crate) fn new(dimensions: dimensions::PackageDimensions, mass: mass::Mass) -> Self {
        Package {
            dimensions,
            mass
        }
    }

    pub(crate) fn sort(&self) -> SortResult {
        match (self.dimensions.classify(), self.mass.classify()) {
            (DimensionClass::BULKY, MassClass::HEAVY) => SortResult::Rejected,
            (DimensionClass::BULKY, _) => SortResult::Special,
            (_, MassClass::HEAVY) => SortResult::Special,
            _ => SortResult::Standard
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::package::dimensions::test_dependencies::{BulkyPackageDimensions, StandardPackageDimensions};
    use crate::package::mass::test_dependencies::{HeavyMass, StandardMass};
    use crate::package::{dimensions, mass, Package};
    use crate::sort_result::SortResult;

    fn package<T, S>(dimensions: T, mass: S) -> Package
    where
        T: Into<dimensions::PackageDimensions>,
        S: Into<mass::Mass>
    {
        Package::new(dimensions.into(), mass.into())
    }

    #[quickcheck]
    fn bulky_and_heavy_items_are_rejected(dimensions: BulkyPackageDimensions, mass: HeavyMass) -> bool {
        package(dimensions, mass).sort() == SortResult::Rejected
    }

    #[quickcheck]
    fn bulky_but_not_heavy_items_are_special(dimensions: BulkyPackageDimensions, mass: StandardMass) -> bool {
        package(dimensions, mass).sort() == SortResult::Special
    }

    #[quickcheck]
    fn heavy_but_not_bulky_items_are_special(dimensions: StandardPackageDimensions, mass: HeavyMass) -> bool {
        package(dimensions, mass).sort() == SortResult::Special
    }

    #[quickcheck]
    fn packages_with_standard_dimensions_and_weight_are_standard(dimensions: StandardPackageDimensions, mass: StandardMass) -> bool {
        package(dimensions, mass).sort() == SortResult::Standard
    }
}
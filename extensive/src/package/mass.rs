use crate::measurements::kg::Kg;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum MassClass {
    HEAVY,
    STANDARD
}

#[derive(Clone, Debug)]
pub(crate) struct Mass {
    value: Kg
}

impl Mass {
    pub(crate) fn new(value: Kg) -> Self {
        Mass {
            value
        }
    }

    pub(crate) fn classify(&self) -> MassClass {
        if self.value >= Kg::new(20).unwrap() {
            MassClass::HEAVY
        } else {
            MassClass::STANDARD
        }
    }
}

#[cfg(test)]
pub(crate) mod test_dependencies {
    use crate::package::mass::Mass;
    use quickcheck::{Arbitrary, Gen};
    use rand::Rng;
    use std::ops::Deref;
    use crate::measurements::kg::Kg;

    #[derive(Clone, Debug)]
    pub(crate) struct StandardMass {
        mass: Mass
    }

    impl From<StandardMass> for Mass {
        fn from(standard_mass: StandardMass) -> Self {
            standard_mass.mass
        }
    }

    impl Deref for StandardMass {
        type Target = Mass;

        fn deref(&self) -> &Self::Target {
            &self.mass
        }
    }

    impl Arbitrary for StandardMass {
        fn arbitrary(_g: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();
            StandardMass {
                mass: Mass::new(Kg::new(rng.gen_range(1..=19)).unwrap())
            }
        }
    }

    #[derive(Clone, Debug)]
    pub(crate) struct HeavyMass {
        mass: Mass
    }

    impl From<HeavyMass> for Mass {
        fn from(heavy_mass: HeavyMass) -> Self {
            heavy_mass.mass
        }
    }

    impl Deref for HeavyMass {
        type Target = Mass;

        fn deref(&self) -> &Self::Target {
            &self.mass
        }
    }

    impl Arbitrary for HeavyMass {
        fn arbitrary(_g: &mut Gen) -> Self {
            let mut rng = rand::thread_rng();
            HeavyMass {
                mass: Mass::new(Kg::new(rng.gen_range(20..=usize::MAX)).unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::mass::test_dependencies::{HeavyMass, StandardMass};

    #[quickcheck]
    fn arbitrary_standard_mass_should_be_less_than_20(mass: StandardMass) -> bool {
        mass.value >= Kg::new(1).unwrap() && mass.value <= Kg::new(19).unwrap()
    }

    #[quickcheck]
    fn arbitrary_heavy_mass_should_be_20_or_more(mass: HeavyMass) -> bool {
        mass.value >= Kg::new(20).unwrap()
    }

    #[quickcheck]
    fn a_heavy_package_is_classified_as_heavy(mass: HeavyMass) -> bool {
        mass.classify() == MassClass::HEAVY
    }

    #[quickcheck]
    fn a_standard_package_is_classified_as_standard(mass: StandardMass) -> bool {
        mass.classify() == MassClass::STANDARD
    }
}
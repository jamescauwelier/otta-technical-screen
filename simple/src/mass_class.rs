pub(crate) enum MassClass {
    HEAVY, STANDARD
}

impl MassClass {
    pub(crate) fn from(mass: usize) -> MassClass {
        if mass >= 20 {
            MassClass::HEAVY
        } else {
            MassClass::STANDARD
        }
    }
}
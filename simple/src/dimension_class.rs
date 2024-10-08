pub(crate) enum DimensionClass {
    STANDARD, BULKY
}

impl DimensionClass {
    pub(crate) fn from(width: usize, height: usize, length: usize) -> DimensionClass {
        let total_dimension = width.saturating_add(height).saturating_add(length);
        if total_dimension >= 150 {
            DimensionClass::BULKY
        } else {
            DimensionClass::STANDARD
        }
    }
}
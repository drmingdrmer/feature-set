use crate::feature::Feature;

/// A required feature with a requirement level.
///
/// A required feature is either mandatory or optional.
#[derive(Debug, Clone)]
pub struct Require {
    pub(crate) optional: bool,
    pub(crate) feature: Feature,
}

impl Require {
    pub fn feature(&self) -> &Feature {
        &self.feature
    }

    pub fn optional(&self) -> bool {
        self.optional
    }
}

impl From<Feature> for Require {
    fn from(feature: Feature) -> Self {
        Self {
            optional: false,
            feature,
        }
    }
}

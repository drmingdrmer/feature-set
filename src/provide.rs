use crate::feature::Feature;

/// A provided feature.
#[derive(Debug, Clone)]
pub struct Provide {
    pub(crate) feature: Feature,
}

impl Provide {
    pub fn feature(&self) -> &Feature {
        &self.feature
    }
}

impl From<Feature> for Provide {
    fn from(feature: Feature) -> Self {
        Self { feature }
    }
}

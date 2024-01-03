use std::collections::BTreeMap;
use std::fmt;

use crate::action::Action;
use crate::feature::Feature;
use crate::Provide;
use crate::Require;

/// A collection of features, indexed by `(feature_name, feature_version)`.
#[derive(Debug, Clone)]
pub struct FeatureSet {
    features: BTreeMap<(&'static str, u64), Feature>,
}

impl FeatureSet {
    /// Create a new `FeatureSet` from a sequence of actions that add or remove a provided feature.
    pub fn from_provides<'a>(provides: impl IntoIterator<Item = &'a Action<Provide>>) -> Self {
        let mut features = BTreeMap::new();
        for a in provides {
            match a {
                Action::Add(p) => {
                    let feature = p.feature();
                    let key = (feature.name(), feature.ver());

                    assert!(
                        !features.contains_key(&key),
                        "duplicate feature: {:?}",
                        feature
                    );

                    features.insert(key, feature.clone());
                }
                Action::Delete(p) => {
                    let feature = p.feature();
                    let key = (feature.name(), feature.ver());

                    assert!(
                        features.contains_key(&key),
                        "feature not found: {:?}",
                        feature
                    );

                    features.remove(&key);
                }
            }
        }

        Self { features }
    }

    /// Create a new `FeatureSet` from a sequence of actions that add or remove a required feature.
    ///
    /// If `include_optional` is `true`, then optional features are included in the resulting
    /// `FeatureSet`.
    pub fn from_required<'a>(
        required: impl IntoIterator<Item = &'a Action<Require>>,
        include_optional: bool,
    ) -> Self {
        let mut features = BTreeMap::new();
        for a in required {
            match a {
                Action::Add(p) => {
                    if !include_optional && p.optional() {
                        continue;
                    }

                    let feature = p.feature();
                    let key = (feature.name(), feature.ver());

                    assert!(
                        !features.contains_key(&key),
                        "duplicate feature: {:?}",
                        feature
                    );

                    features.insert(key, feature.clone());
                }
                Action::Delete(p) => {
                    let feature = p.feature();
                    let key = (feature.name(), feature.ver());

                    assert!(
                        features.contains_key(&key),
                        "feature not found: {:?}",
                        feature
                    );

                    features.remove(&(feature.name(), feature.ver()));
                }
            }
        }

        Self { features }
    }

    pub fn contains(&self, name_ver: (&str, u64)) -> bool {
        self.features.contains_key(&name_ver)
    }
}

impl fmt::Display for FeatureSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.features
                .keys()
                .map(|(n, v)| { format!("{}:v{:}", n, v) })
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

use semver::Version;

/// `Since` describes when a feature is added or removed.
#[derive(Debug, Clone)]
pub struct Since {
    /// The date when a feature is added or removed.
    pub(crate) date: &'static str,

    /// The version when a feature is added or removed.
    pub(crate) version: Version,
}

impl Since {
    /// Create a new `Since` instance.
    pub const fn new(date: &'static str, ver: (u64, u64, u64)) -> Self {
        Self {
            date,
            version: Version::new(ver.0, ver.1, ver.2),
        }
    }

    /// Get the date when a feature is added or removed.
    pub fn date(&self) -> &'static str {
        self.date
    }

    /// Get the version when a feature is added or removed.
    pub fn version(&self) -> &Version {
        &self.version
    }
}

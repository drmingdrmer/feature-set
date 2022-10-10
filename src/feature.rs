use crate::Since;

/// A named, versioned feature that can be provided or required.
///
/// A `Feature` is uniquely identified by its name and version.
/// A `Feature` tracks the date it was introduced and the version of the crate it was introduced in.
#[derive(Debug, Clone)]
pub struct Feature {
    pub(crate) name: &'static str,
    pub(crate) ver: u64,
    pub(crate) since: Since,
}

impl Feature {
    pub const fn new(
        name_ver: (&'static str, u64),
        since_date: &'static str,
        since_ver: (u64, u64, u64),
    ) -> Self {
        Self {
            name: name_ver.0,
            ver: name_ver.1,
            since: Since::new(since_date, since_ver),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn ver(&self) -> u64 {
        self.ver
    }

    pub fn since(&self) -> &Since {
        &self.since
    }
}

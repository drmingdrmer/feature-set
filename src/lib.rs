//! Tracks features provided/required by a library.
//!
//! Example:
//! ```
//! # use feature_set::{Action, add_provide, del_provide, FeatureSet, Provide};
//! const PROVIDED: &[Action<Provide>] = &[
//!     add_provide(("foo", 0), "2012-02-21", (1, 2, 3)),
//!     add_provide(("foo", 1), "2012-02-22", (1, 2, 3)),
//!     add_provide(("bar", 1), "2012-03-21", (1, 2, 4)),
//!     del_provide(("foo", 0), "2012-04-21", (1, 2, 5)),
//! ];
//!
//! let fs = FeatureSet::from_provides(PROVIDED);
//! assert_eq!("bar:v1, foo:v1", fs.to_string());
//! ```

mod action;
mod feature;
mod feature_set;
mod provide;
mod require;
mod since;

pub use crate::action::Action;
pub use crate::feature::Feature;
pub use crate::feature_set::FeatureSet;
pub use crate::provide::Provide;
pub use crate::require::Require;
pub use crate::since::Since;

#[cfg(test)]
mod tests;

/// Create an `Add a new provided feature` action.
pub const fn add_provide(
    name_ver: (&'static str, u64),
    since_date: &'static str,
    since_ver: (u64, u64, u64),
) -> Action<Provide> {
    Action::Add(Provide {
        feature: Feature::new(name_ver, since_date, since_ver),
    })
}

/// Create a `Remove a provided feature` action.
pub const fn del_provide(
    name_ver: (&'static str, u64),
    date: &'static str,
    ver: (u64, u64, u64),
) -> Action<Provide> {
    Action::Delete(Provide {
        feature: Feature::new(name_ver, date, ver),
    })
}

pub const fn add_require(
    name: (&'static str, u64),
    date: &'static str,
    ver: (u64, u64, u64),
) -> Action<Require> {
    Action::Add(Require {
        optional: false,
        feature: Feature::new(name, date, ver),
    })
}

pub const fn del_require(
    name: (&'static str, u64),
    date: &'static str,
    ver: (u64, u64, u64),
) -> Action<Require> {
    Action::Delete(Require {
        optional: false,
        feature: Feature::new(name, date, ver),
    })
}

pub const fn add_optional(
    name: (&'static str, u64),
    date: &'static str,
    ver: (u64, u64, u64),
) -> Action<Require> {
    Action::Add(Require {
        optional: true,
        feature: Feature::new(name, date, ver),
    })
}

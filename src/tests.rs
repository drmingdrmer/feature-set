use crate::action::Action;
use crate::add_optional;
use crate::add_provide;
use crate::add_require;
use crate::del_provide;
use crate::del_require;
use crate::provide::Provide;
use crate::FeatureSet;
use crate::Require;

const PROVIDED: &[Action<Provide>] = &[
    add_provide(("foo", 0), "2012-02-21", (1, 2, 3)),
    add_provide(("bar", 1), "2012-02-21", (1, 2, 3)),
    add_provide(("baz", 0), "2012-02-21", (1, 2, 3)),
    add_provide(("baz", 1), "2012-02-21", (1, 2, 3)),
    add_provide(("qux", 1), "2012-02-21", (1, 2, 3)),
    del_provide(("baz", 0), "2012-02-21", (1, 2, 3)),
];

#[test]
fn test_provide() {
    let fs = FeatureSet::from_provides(PROVIDED);
    assert_eq!("bar:v1, baz:v1, foo:v0, qux:v1", fs.to_string());
}

const REQUIRED: &[Action<Require>] = &[
    add_require(("foo", 0), "2012-02-21", (1, 2, 3)),
    add_require(("bar", 0), "2012-02-21", (1, 2, 3)),
    add_optional(("bar", 1), "2012-02-21", (1, 2, 3)),
    del_require(("bar", 0), "2012-02-21", (1, 2, 3)),
];

#[test]
fn test_require() {
    let fs = FeatureSet::from_required(REQUIRED, false);
    assert_eq!("foo:v0", fs.to_string());

    let fs = FeatureSet::from_required(REQUIRED, true);
    assert_eq!("bar:v1, foo:v0", fs.to_string());
}

/// An action that adds or removes a provided/required feature.
///
/// Example:
/// ```
/// # use feature_set::{Action, add_provide, del_provide, FeatureSet, Provide};
/// const PROVIDED: &[Action<Provide>] = &[
///     add_provide(("foo", 0), "2012-02-21", (1, 2, 3)),
///     add_provide(("foo", 1), "2012-02-22", (1, 2, 3)),
///     add_provide(("bar", 1), "2012-03-21", (1, 2, 4)),
///     del_provide(("foo", 0), "2012-04-21", (1, 2, 5)),
/// ];
///
/// let fs = FeatureSet::from_provides(PROVIDED);
/// assert_eq!("bar:v1, foo:v1", fs.to_string());
/// ```
#[derive(Debug, Clone)]
pub enum Action<T> {
    Add(T),
    Delete(T),
}

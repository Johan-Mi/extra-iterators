#![forbid(unsafe_code, missing_docs)]
#![warn(clippy::nursery, clippy::pedantic)]
#![doc = include_str!("../README.md")]
#![no_std]

/// A version of [`from_fn`] that takes a fallible closure.
///
/// [`from_fn`]: core::iter::from_fn
pub fn try_from_fn<T, E>(
    mut f: impl FnMut() -> Result<Option<T>, E>,
) -> impl Iterator<Item = Result<T, E>> {
    core::iter::from_fn(move || f().transpose())
}

/// A version of [`map`] that also gives its closure access to the entire
/// iterator, so that each element of the output can be derived from more than
/// one element of the input.
///
/// [`map`]: Iterator::map
pub fn batching_map<I: Iterator, O>(
    mut it: I,
    mut f: impl FnMut(&mut I, I::Item) -> O,
) -> impl Iterator<Item = O> {
    core::iter::from_fn(move || it.next().map(|t| f(&mut it, t)))
}

use std::ops::Deref;

/// Score-qualified type returned as a search result.
#[derive(Debug, PartialEq)]
pub struct Match<'a, T: 'a> {
    object: &'a T,
    pub score: f32,
}

impl<'a, T> Match<'a, T> {
    pub fn new(object: &'a T, score: f32) -> Match<'a, T> {
        Match{ object: object, score: score }
    }
}

impl<'a, T> Deref for Match<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.object
    }
}

/// Borrowed string slice representation used for matching.
pub trait AsStr {
    fn as_str(&self) -> &str;
}

impl AsStr for String {
    fn as_str(&self) -> &str {
        self.as_str()
    }
}

impl<'a> AsStr for &'a str {
    fn as_str(&self) -> &str {
        self
    }
}

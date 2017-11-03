use std::ops::Deref;

/// Score-qualified type returned as a search result.
#[derive(Debug, PartialEq)]
pub struct Match<T> {
    object: T,
    pub score: f32,
}

impl<T> Match<T> {
    pub fn new(object: T, score: f32) -> Match<T> {
        Match{ object: object, score: score }
    }
}

impl<T> Deref for Match<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.object
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

impl<'a> AsStr for &'a String {
    fn as_str(&self) -> &str {
        self
    }
}

impl<'a> AsStr for &'a str {
    fn as_str(&self) -> &str {
        self
    }
}

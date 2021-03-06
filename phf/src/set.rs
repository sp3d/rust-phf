//! An immutable set constructed at compile time.
use core::prelude::*;
use core::borrow::BorrowFrom;
use core::fmt;

use PhfHash;
use map;
use Map;

/// An immutable set constructed at compile time.
///
/// `Set`s may be created with the `phf_set` macro:
///
/// ```rust
/// # #![feature(phase)]
/// extern crate phf;
/// #[phase(plugin)]
/// extern crate phf_mac;
///
/// static MY_SET: phf::Set<&'static str> = phf_set! {
///    "hello",
///    "world",
/// };
///
/// # fn main() {}
/// ```
///
/// ## Note
///
/// The fields of this struct are public so that they may be initialized by the
/// `phf_set` macro. They are subject to change at any time and should never be
/// accessed directly.
pub struct Set<T:'static> {
    #[doc(hidden)]
    pub map: Map<T, ()>
}

impl<T> fmt::Show for Set<T> where T: fmt::Show {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "{{"));
        let mut first = true;
        for entry in self.iter() {
            if !first {
                try!(write!(fmt, ", "));
            }
            try!(write!(fmt, "{}", entry));
            first = false;
        }
        write!(fmt, "}}")
    }
}

impl<T> Set<T> {
    /// Returns the number of elements in the `Set`.
    pub fn len(&self) -> uint {
        self.map.len()
    }

    /// Returns true if the `Set` contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a reference to the set's internal static instance of the given
    /// key.
    ///
    /// This can be useful for interning schemes.
    pub fn get_key<Sized? U>(&self, key: &U) -> Option<&T> where U: Eq + PhfHash + BorrowFrom<T> {
        self.map.get_key(key)
    }

    /// Returns true if `value` is in the `Set`.
    pub fn contains<Sized? U>(&self, value: &U) -> bool where U: Eq + PhfHash + BorrowFrom<T> {
        self.map.contains_key(value)
    }

    /// Returns an iterator over the values in the set.
    ///
    /// Values are returned in an arbitrary but fixed order.
    pub fn iter<'a>(&'a self) -> Items<'a, T> {
        Items { iter: self.map.keys() }
    }
}

impl<T> Set<T> where T: Eq + PhfHash {
    /// Returns true if `other` shares no elements with `self`.
    pub fn is_disjoint(&self, other: &Set<T>) -> bool {
        !self.iter().any(|value| other.contains(value))
    }

    /// Returns true if `other` contains all values in `self`.
    pub fn is_subset(&self, other: &Set<T>) -> bool {
        self.iter().all(|value| other.contains(value))
    }

    /// Returns true if `self` contains all values in `other`.
    pub fn is_superset(&self, other: &Set<T>) -> bool {
        other.is_subset(self)
    }
}

/// An iterator over the values in a `Set`.
pub struct Items<'a, T:'static> {
    iter: map::Keys<'a, T, ()>,
}

impl<'a, T> Iterator<&'a T> for Items<'a, T> {
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator<&'a T> for Items<'a, T> {
    fn next_back(&mut self) -> Option<&'a T> {
        self.iter.next_back()
    }
}

impl<'a, T> ExactSizeIterator<&'a T> for Items<'a, T> {}



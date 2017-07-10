//! Implements functionality similar to the built-in 'slices',
//! but for any type that implements `Index` and/or `IndexMut`
//!
//! This is useful for built-in types such as `VecDeque`
//! which cannot use the built-in slices (`[T]`) because the memory isn't
//! laid out contiguously. It is also convenient for your own custom types -
//! you can just implement `Index` and `IndexMut` and then you receive a
//! slicing implementation for free.
//!
//! # Implementing `TakeSlice` for your own types
//!
//! ```
//! use std::ops::{Index, IndexMut};
//! use std::collections::VecDeque;
//! use owned_slice::TakeSlice;
//!
//! // Let's pretend we want to be able to take 'slices' from our CustomStruct.
//! pub struct CustomStruct<T> {
//!     secret: VecDeque<T>
//! }
//!
//! impl<T> Index<usize> for CustomStruct<T> {
//!     type Output = T;
//!     fn index(&self, index: usize) -> &T {
//!         &self.secret[index+1]
//!     }
//! }
//!
//! impl<T> IndexMut<usize> for CustomStruct<T> {
//!     fn index_mut(&mut self, index: usize) -> &mut T {
//!         &mut self.secret[index + 1]
//!     }
//! }
//!
//! // Now we simply implement this trait for our custom data structure,
//! // and we receive functionality similar to built-in slices for free!
//! impl<T> TakeSlice<T, usize> for CustomStruct<T> {
//!     // this value doesn't really matter,
//!     // since bounds checks are still included in the `Index` and `IndexMut` impls,
//!     // however this will give nicer error messages.
//!     fn len(&self) -> usize {
//!         self.secret.len() - 1
//!     }
//! }
//!
//! let mut inner = VecDeque::new();
//! inner.push_back("foo");
//! inner.push_back("bar");
//! inner.push_back("baz");
//! assert_eq!(inner.index_range_from(1..)[0], "bar");
//! let custom = CustomStruct {
//!     secret: inner
//! };
//!
//! // remember that our custom struct adds `1` onto every index
//! assert_eq!(custom.index_range_to(..2)[1], "baz");
//!
//!
//!
//! ```
//!

extern crate num;

mod iter;
mod util;

use std::collections::VecDeque;
use std::ops::{Add, Sub, Range, RangeTo, RangeFrom, Index, IndexMut};
use std::cmp::{Eq, Ord};
use std::fmt::Debug;
use std::marker;
use num::{Zero, One};

pub use iter::{Iter, IterMut};
use util::{unlikely, assert_in_bounds};

/// This trait looks similar to the `Num` trait from `num`, however it doesn't
/// require things like `Mul`, `Div`, `Rem` and `from_str_radix`.
/// In addition, it is automatically implemented, whereas you'd have to implement `Num` manually.
pub trait Idx
    : Add<Self, Output = Self> + Sub<Self, Output = Self> + Zero + One + Eq + Ord + Debug + Copy
    {
}

impl<T: Add<Self, Output=Self>
      + Sub<Self, Output=Self>
      + Zero + One + Eq + Ord
      + Debug + Copy> Idx for T {}

// Immutable Version
#[derive(Copy, Clone, Debug)]
pub struct Slice<'a, K: 'a + Index<I, Output = T>, I: 'a + Idx, T: 'a> {
    list: &'a K,
    start: I,
    len: I,
    ty: marker::PhantomData<T>,
}

impl<'a, K, I, T> Slice<'a, K, I, T>
    where K: Index<I, Output = T>,
          I: Idx
{
    pub fn new(list: &'a K, index: Range<I>) -> Slice<'a, K, I, T> {
        Slice {
            list: list,
            start: index.start,
            len: index.end - index.start,
            ty: marker::PhantomData,
        }
    }

    pub fn iter(self) -> Iter<'a, K, I, T> {
        Iter::new(self)
    }
}

impl<'a, K, I, T> Index<I> for Slice<'a, K, I, T>
    where K: Index<I, Output = T>,
          I: Idx
{
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &T {
        if unlikely(index >= self.len) {
            panic!("Index out of bounds: {:?} >= {:?}", index, self.len);
        }
        &self.list[self.start + index]
    }
}

// Mutable Version
pub struct SliceMut<'a, K: 'a + IndexMut<I, Output = T>, I: 'a + Idx, T: 'a> {
    list: &'a mut K,
    start: I,
    len: I,
    ty: marker::PhantomData<T>,
}

impl<'a, K, I, T> SliceMut<'a, K, I, T>
    where K: IndexMut<I, Output = T>,
          I: Idx
{
    pub fn new(list: &'a mut K, index: Range<I>) -> SliceMut<'a, K, I, T> {
        SliceMut {
            list: list,
            start: index.start,
            len: index.end - index.start,
            ty: marker::PhantomData,
        }
    }

    pub fn iter_mut(self) -> IterMut<'a, K, I, T> {
        IterMut::new(self)
    }
}

impl<'a, K, I, T> Index<I> for SliceMut<'a, K, I, T>
    where K: IndexMut<I, Output = T>,
          I: Idx
{
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &T {
        if unlikely(index >= self.len) {
            panic!("Index out of bounds: {:?} >= {:?}", index, self.len);
        }
        &self.list[self.start + index]
    }
}

impl<'a, K, I, T> IndexMut<I> for SliceMut<'a, K, I, T>
    where K: IndexMut<I, Output = T>,
          I: Idx
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut T {
        if unlikely(index >= self.len) {
            panic!("Index out of bounds: {:?} >= {:?}", index, self.len);
        }
        &mut self.list[self.start + index]
    }
}

///  Mimics the built in slices [T] for various built-in types
/// and also your own custom data structures.
pub trait TakeSlice<T, I>: Index<I, Output = T> + IndexMut<I> + Sized
    where I: Idx
{
    /// Slice the structure with a range.
    /// Equivalent to `&container[start..end]`
    fn index_range(&self, index: Range<I>) -> Slice<Self, I, T> {
        assert_in_bounds(&index, self.len());
        Slice {
            list: self,
            start: index.start,
            len: index.end - index.start,
            ty: marker::PhantomData,
        }
    }

    /// Slice the structure with a range, returning a mutable reference.
    /// Equivalent to `&mut container[start..end]`
    fn index_range_mut(&mut self, index: Range<I>) -> SliceMut<Self, I, T> {
        assert_in_bounds(&index, self.len());
        SliceMut {
            list: self,
            start: index.start,
            len: index.end - index.start,
            ty: marker::PhantomData,
        }
    }

    /// Slice the structure from the beginning to the specified index.
    /// Equivalent to `&container[..end]`
    fn index_range_to(&self, index: RangeTo<I>) -> Slice<Self, I, T> {
        self.index_range(Zero::zero()..index.end)
    }

    /// Slice the structure from the beginning to the specified index,
    /// returning a mutable reference.
    /// Equivalent to `&mut container[..end]`
    fn index_range_to_mut(&mut self, index: RangeTo<I>) -> SliceMut<Self, I, T> {
        self.index_range_mut(Zero::zero()..index.end)
    }

    /// Slice the structure from the specified index to the end.
    /// Equivalent to `&container[start..]`
    fn index_range_from(&self, index: RangeFrom<I>) -> Slice<Self, I, T> {
        let len = self.len();
        self.index_range(index.start..len)
    }

    /// Slice the structure from the specified index to the end,
    /// returning a mutable reference.
    /// Equivalent to `&mut container[start..]`
    fn index_range_from_mut(&mut self, index: RangeFrom<I>) -> SliceMut<Self, I, T> {
        let len = self.len();
        self.index_range_mut(index.start..len)
    }

    /// Returns the number of elements in the container.
    /// Used for providing nicer out-of-bounds errors.
    fn len(&self) -> I;
}

impl<T> TakeSlice<T, usize> for VecDeque<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use TakeSlice;

    fn test_vec() -> VecDeque<usize> {
        let mut v = VecDeque::new();
        v.push_back(0);
        v.push_back(1);
        v.push_back(2);
        v.push_back(3);
        v.push_back(4);
        v
    }

    #[test]
    fn basic_slice_functionality() {
        let v = test_vec();
        let v = v.index_range(1..3);
        let mut iter = v.clone().iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_check() {
        let v = test_vec();
        let v = v.index_range(1..4);
        println!("{:?}", v[3]);
    }
}

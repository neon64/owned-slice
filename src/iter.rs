use std::ops::{Index, IndexMut};
use std::fmt::Debug;
use std::marker;
use num_traits::One;
use super::{Idx, Slice, SliceMut};

impl<'a, K, I, T> IntoIterator for Slice<'a, K, I, T>
    where K: Index<I, Output = T>,
          I: Idx
{
    type Item = &'a T;
    type IntoIter = Iter<'a, K, I, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

/// The iterator for an immutable slice.
pub struct Iter<'a, K: 'a + Index<I, Output = T>, I: 'a + Idx, T: 'a> {
    list: &'a K,
    cur: I,
    end: I,
    ty: marker::PhantomData<T>,
}

impl<'a, K, I, T> Iter<'a, K, I, T>
    where K: Index<I, Output = T>,
          I: Idx + Debug
{
    pub fn new(slice: Slice<'a, K, I, T>) -> Self {
        Iter {
            list: slice.list,
            cur: slice.start,
            end: slice.start + slice.len,
            ty: marker::PhantomData,
        }
    }
}

impl<'a, K, I, T> Iterator for Iter<'a, K, I, T>
    where K: Index<I, Output = T>,
          I: Idx
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur {
            x if x == self.end => None,
            _ => {
                let item = &self.list[self.cur];
                self.cur = self.cur + One::one();
                Some(item)
            }
        }
    }
}

impl<'a, K, I, T> IntoIterator for SliceMut<'a, K, I, T>
    where K: IndexMut<I, Output = T>,
          I: Idx
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, K, I, T>;
    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self)
    }
}

/// The iterator for a mutable slice.
pub struct IterMut<'a, K: 'a + IndexMut<I, Output = T>, I: 'a + Idx, T: 'a> {
    list: &'a mut K,
    cur: I,
    end: I,
    ty: marker::PhantomData<T>,
}

impl<'a, K, I, T> IterMut<'a, K, I, T>
    where K: IndexMut<I, Output = T>,
          I: Idx
{
    pub fn new(slice: SliceMut<'a, K, I, T>) -> Self {
        IterMut {
            list: slice.list,
            cur: slice.start,
            end: slice.start + slice.len,
            ty: marker::PhantomData,
        }
    }
}

impl<'a, K, I, T> Iterator for IterMut<'a, K, I, T>
    where K: IndexMut<I, Output = T>,
          I: Idx
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur {
            x if x == self.end => None,
            _ => {
                let item = &mut self.list[self.cur];
                // let's skip borrowck here just like `std` does :D
                // PS: I hope its safe!
                let item = unsafe { &mut *(item as *mut _) };
                self.cur = self.cur + One::one();
                Some(item)
            }
        }
    }
}

use num::Zero;
use super::Idx;
use std::ops::Range;

#[cfg(not(feature = "nightly"))]
#[inline(always)]
#[cfg_attr(feature = "clippy", allow(inline_always))]
pub fn unlikely(x: bool) -> bool {
    x
}

#[cfg(feature = "nightly")]
#[inline(always)]
#[cfg_attr(feature = "clippy", allow(inline_always))]
pub fn unlikely(x: bool) -> bool {
    unsafe { ::std::intrinsics::unlikely(x) }
}

#[inline]
pub fn assert_in_bounds<I: Idx>(index: &Range<I>, len: I) {
    if unlikely(index.end > len) {
        panic!("Range out of bounds: {:?} is not a subset of {:?}",
               index,
               Zero::zero()..len);
    }
}

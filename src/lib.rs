#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::cmp::Ordering::*;

fn std_binary_search_by<'a, T, F>(slice: &'a [T], mut f: F) -> Result<usize, usize>
where
    T: Ord,
    F: FnMut(&'a T) -> Ordering,
{
    let s = slice;
    let mut size = s.len();
    if size == 0 {
        return Err(0);
    }
    let mut base = 0usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        // SAFETY: the call is made safe by the following inconstants:
        // - `mid >= 0`: by definition
        // - `mid < size`: `mid = size / 2 + size / 4 + size / 8 ...`
        let cmp = f(unsafe { s.get_unchecked(mid) });
        base = if cmp == Greater { base } else { mid };
        size -= half;
    }
    // SAFETY: base is always in [0, size) because base <= mid.
    let cmp = f(unsafe { s.get_unchecked(base) });
    if cmp == Equal {
        Ok(base)
    } else {
        Err(base + (cmp == Less) as usize)
    }
}

pub fn std_binary_search<'a, T: Ord>(slice: &'a [T], x: &T) -> Result<usize, usize> {
    std_binary_search_by(slice, |p| p.cmp(x))
}

fn custom_binary_search_by_1<'a, T, F>(slice: &'a [T], mut f: F) -> Result<usize, usize>
where
    T: Ord,
    F: FnMut(&'a T) -> Ordering,
{
    let s = slice;
    let mut left = 0;
    let mut right = s.len();
    while left < right {
        // never overflow because `slice::len()` max is `isize::MAX`.
        let mid = (left + right) / 2;
        // SAFETY: the call is made safe by the following invariants:
        // - `mid >= 0`
        // - `mid < size`: `mid` is limited by `[left; right)` bound.
        let cmp = f(unsafe { s.get_unchecked(mid) });
        if cmp == Less {
            left = mid + 1;
        } else if cmp == Greater {
            right = mid;
        } else {
            return Ok(mid);
        }
    }
    Err(left)
}

pub fn custom_binary_search_1<'a, T: Ord>(slice: &'a [T], x: &T) -> Result<usize, usize> {
    custom_binary_search_by_1(slice, |p| p.cmp(x))
}

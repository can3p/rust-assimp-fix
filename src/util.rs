use std::slice;
use std::mem;

#[inline(always)]
pub unsafe fn ptr_ptr_to_slice<'a, T>(ptr: *mut*mut T, len: usize) -> &'a [&'a T] {
    mem::transmute(slice::from_raw_parts(ptr, len as usize))
}

#[inline(always)]
pub unsafe fn ptr_to_slice<'a, T>(ptr: *mut T, len: usize) -> &'a [T] {
    slice::from_raw_parts(ptr, len)
}

use core::alloc::{GlobalAlloc, Layout};

extern "C" {
    fn aligned_alloc(alignment: usize, size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

/// Just as it says on the tin.
/// A GlobalAlloc implementation that calls `aligned_alloc` and `free`.
pub struct Mallocator;

unsafe impl GlobalAlloc for Mallocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        aligned_alloc(layout.align(), layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr)
    }
}

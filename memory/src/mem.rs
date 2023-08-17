use crate::Pod;
use std::mem::size_of;

/// A view into the x86 memory.
/// Basically a slice, but using pointers to:
/// 1. make accesses more explicit
/// 2. handle unaligned reads
/// 3. expose casts to/from Pod types
///
/// TODO: some of the accessors on here that return references likely violate
/// Rust rules around aliasing.  It might be better to instead always copy from
/// memory, rather than aliasing into it.
#[derive(Copy, Clone)]
pub struct Mem<'m> {
    ptr: *mut u8,
    end: *mut u8,
    _marker: std::marker::PhantomData<&'m u8>,
}

impl<'m> Mem<'m> {
    pub fn from_slice(s: &'m [u8]) -> Mem<'m> {
        let range = s.as_ptr_range();
        Mem {
            ptr: range.start as *mut u8,
            end: range.end as *mut u8,
            _marker: std::marker::PhantomData::default(),
        }
    }

    pub fn get<T: Clone + Pod>(&self, ofs: u32) -> T {
        unsafe {
            let ptr = self.ptr.add(ofs as usize);
            if ptr.add(size_of::<T>()) > self.end {
                panic!("oob");
            }
            std::ptr::read_unaligned(ptr as *const T)
        }
    }

    pub fn put<T: Copy + Pod>(&self, ofs: u32, val: T) {
        // TODO: alignment?
        *self.view_mut(ofs) = val;
    }

    pub fn slicez(&self, ofs: u32) -> Option<Mem<'m>> {
        let slice = self.as_slice_todo();
        let nul = slice[ofs as usize..].iter().position(|&c| c == 0)? as u32;
        Some(self.sub(ofs, nul))
    }

    pub fn to_ascii(&self) -> &'m str {
        let slice = self.as_slice_todo();
        match std::str::from_utf8(slice) {
            Ok(str) => str,
            Err(err) => {
                // If we hit one of these, we ought to change the caller to not use to_ascii().
                panic!("failed to ascii convert {:?}: {}", slice, err);
            }
        }
    }

    pub fn as_slice_todo(&self) -> &'m [u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len() as usize) }
    }

    /// TODO: don't expose slices of memory, as we might not have contiguous pages.
    pub fn as_mut_slice_todo(&self) -> &'m mut [u8] {
        unsafe {
            let len = self.end.offset_from(self.ptr) as usize;
            std::slice::from_raw_parts_mut(self.ptr, len)
        }
    }

    pub fn len(&self) -> u32 {
        unsafe { self.end.offset_from(self.ptr) as u32 }
    }

    pub fn slice(&self, b: impl std::ops::RangeBounds<u32>) -> Mem<'m> {
        unsafe {
            let ptr = self.ptr.add(match b.start_bound() {
                std::ops::Bound::Included(&n) => n as usize,
                std::ops::Bound::Excluded(&n) => n as usize + 1,
                std::ops::Bound::Unbounded => 0,
            });
            let end = self.ptr.add(match b.end_bound() {
                std::ops::Bound::Included(&n) => n as usize + 1,
                std::ops::Bound::Excluded(&n) => n as usize,
                std::ops::Bound::Unbounded => self.len() as usize,
            });
            if !(self.ptr..self.end).contains(&ptr) || !(self.ptr..self.end.add(1)).contains(&end) {
                panic!("oob");
            }
            Mem {
                ptr,
                end,
                _marker: std::marker::PhantomData::default(),
            }
        }
    }

    pub fn sub(&self, ofs: u32, len: u32) -> Mem<'m> {
        self.slice(ofs..(ofs + len))
    }

    // TODO: this fails if addr isn't appropriately aligned.
    // We need to revisit this whole "view" API...
    pub fn view<T: Pod>(&self, addr: u32) -> &'m T {
        unsafe {
            let ptr = self.ptr.add(addr as usize);
            if ptr.add(size_of::<T>()) > self.end {
                panic!("oob");
            }
            &*(ptr as *const T)
        }
    }

    // TODO: this fails if addr isn't appropriately aligned.
    // We need to revisit this whole "view" API...
    pub fn view_mut<T: Pod>(&self, addr: u32) -> &'m mut T {
        unsafe {
            let ptr = self.ptr.add(addr as usize);
            if ptr.add(size_of::<T>()) > self.end {
                panic!("oob");
            }
            &mut *(ptr as *mut T)
        }
    }

    pub fn view_n<T: Pod>(&self, ofs: u32, count: u32) -> &'m [T] {
        unsafe {
            let ptr = self.ptr.add(ofs as usize);
            if ptr.add(size_of::<T>() * count as usize) > self.end {
                panic!("oob");
            }
            std::slice::from_raw_parts(ptr as *const T, count as usize)
        }
    }

    /// Note: can returned unaligned pointers depending on addr.
    pub fn ptr_mut<T: Pod + Copy>(&self, addr: u32) -> *mut T {
        unsafe {
            let ptr = self.ptr.add(addr as usize);
            if ptr.add(size_of::<T>()) > self.end {
                panic!("oob");
            }
            ptr as *mut T
        }
    }
}

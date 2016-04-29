//! Problem:
//!
//! ```
//! pub struct Foo {
//!     ...
//! }
//!
//! impl Foo {
//!     fn do_something(&mut self, ...){
//!         //FIXME: avoid allocation. can't fix this because T is lifetime-bound.
//!         let mut guards: Vec<Guard<'x>> = Vec::with_capacity(xxx.len());
//!         ...
//!     }
//! }
//! ```
//!
//! Solution:
//!
//! ```
//! use tmp_vec::TmpVec;
//!
//! pub struct Foo {
//!     tmp_guards: TmpVec<Guard<'static>>,
//!     ...
//! }
//!
//! impl Foo {
//!     fn do_something(&mut self, ...){
//!          let mut guards = self.tmp_guards.borrow_mut();
//!          ...
//!     }
//! }
//!         
//! ```

use std::ops::{Deref, DerefMut};

pub struct BorrowMut<'a, T: 'a> {
    inner: &'a mut Vec<T>
}

impl<'a, T> Deref for BorrowMut<'a, T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target { self.inner }
}

impl<'a, T> DerefMut for BorrowMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target { self.inner }
}

impl<'a, T> Drop for BorrowMut<'a, T> {
    fn drop(&mut self){
        self.inner.clear();
    }
}

pub struct TmpVec<U> {
    inner: Vec<U>
}

impl<U> Default for TmpVec<U> {
    fn default() -> Self { Self::new() }
}

impl<U> TmpVec<U> {
    pub fn new() -> Self { TmpVec{ inner: Vec::new() } }
    pub fn with_capacity(cap: usize) -> Self { TmpVec{ inner: Vec::with_capacity(cap) } }
    
    pub fn borrow_mut<T>(&mut self) -> BorrowMut<T> {
        use std::mem;
        assert_eq!(mem::size_of::<T>(), mem::size_of::<U>());
        assert_eq!(mem::align_of::<T>(), mem::align_of::<U>());
        unsafe { self.inner.set_len(0); } // leak if BorrowMut was leaked
        BorrowMut{ inner: unsafe { mem::transmute(&mut self.inner) } }
    }
}

impl<U> Drop for TmpVec<U> {
    fn drop(&mut self){
        unsafe { self.inner.set_len(0); } // leak if BorrowMut was leaked
    }
}


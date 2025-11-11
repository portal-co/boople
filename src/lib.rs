#![no_std]
#![cfg_attr(feature = "unsize", feature(unsize))]
#![cfg_attr(
    feature = "allocator-api",
    feature(allocator_api, arbitrary_self_types_pointers)
)]
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use core::mem::MaybeUninit;
#[cfg(feature = "alloc")]
#[doc(hidden)]
pub extern crate alloc;
#[doc(hidden)]
pub mod __ {
    pub use core::marker::Sized;
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(transparent)]
pub struct Shim<T>(pub T);
pub trait Buttons<B> {
    type Result;
    fn push(self, buttons: B) -> Self::Result;
}
#[cfg(feature = "alloc")]
#[cfg(not(feature = "allocator-api"))]
pub trait BoxButtons<B, R: ?Sized> {
    // type Result;
    fn push(self: Box<Self>, buttons: B) -> Box<R>;
}
#[cfg(feature = "alloc")]
#[cfg(feature = "allocator-api")]
pub trait BoxButtons<B, R: ?Sized, A: alloc::alloc::Allocator = alloc::alloc::Global> {
    // type Result;
    fn push(self: Box<Self, A>, buttons: B) -> Box<R, A>;
}
#[cfg(feature = "allocator-api")]
pub trait BoxButtonsShim<B, R: ?Sized, A: alloc::alloc::Allocator> {
    // type Result;
    unsafe fn push(self: *mut Self, allocator: A, buttons: B) -> Box<R, A>;
}
#[cfg(feature = "alloc")]
const _: () = {
    #[cfg(not(feature = "allocator-api"))]
    impl<B, T: Buttons<B, Result: Unsize<R>>, R: ?Sized> BoxButtons<B, R> for T {
        fn push(self: Box<Self>, buttons: B) -> Box<R> {
            Box::new(Buttons::push(*self, buttons)) as Box<R>
        }
    }
    #[cfg(feature = "allocator-api")]
    impl<B, T: Buttons<B, Result: Unsize<R>>, R: ?Sized, A: alloc::alloc::Allocator>
        BoxButtons<B, R, A> for T
    {
        fn push(self: Box<Self, A>, buttons: B) -> Box<R, A> {
            use core::alloc::Layout;

            let (rp, a) = Box::into_raw_with_allocator(self);
            let r = unsafe { core::ptr::read(rp) };
            unsafe {
                use core::ptr::NonNull;
                a.deallocate(NonNull::new_unchecked(rp.cast()), Layout::new::<T>())
            };
            Box::new_in(Buttons::push(r, buttons), a) as Box<R, A>
        }
    }
    #[cfg(feature = "allocator-api")]
    impl<B, R: ?Sized, A: alloc::alloc::Allocator, T: BoxButtons<B, R, A> + ?Sized>
        BoxButtonsShim<B, R, A> for T
    {
        unsafe fn push(self: *mut Self, allocator: A, buttons: B) -> Box<R, A> {
            let _box = unsafe { Box::from_raw_in(self, allocator) };
            BoxButtons::push(_box, buttons)
        }
    }
    use core::marker::Unsize;
    #[cfg(not(feature = "allocator-api"))]
    impl<'a, B, T: Unsize<dyn BoxButtons<B, T> + 'a> + ?Sized + 'a> Buttons<B> for Box<T> {
        type Result = Box<T>;
        fn push(self, buttons: B) -> Self::Result {
            let this: Box<dyn BoxButtons<B, T> + 'a> = self;
            BoxButtons::push(this, buttons)
        }
    }
    #[cfg(feature = "allocator-api")]
    impl<
        'a,
        B,
        A: alloc::alloc::Allocator,
        T: Unsize<dyn BoxButtonsShim<B, T, A> + 'a> + ?Sized + 'a,
    > Buttons<B> for Box<T, A>
    {
        type Result = Box<T, A>;
        fn push(self, buttons: B) -> Self::Result {
            let this: Box<dyn BoxButtonsShim<B, T, A> + 'a, A> = self;
            let (a, b) = Box::into_raw_with_allocator(this);
            unsafe { BoxButtonsShim::push(a, b, buttons) }
        }
    }
};
#[macro_export]
macro_rules! buttons {
    (<$($t:tt)*> $i:ident => [$($a:ty),*]) => {
        pub trait $i<$($t)*>: $($crate::Buttons<$a, Result: $i> + )* $crate::__::Sized{
        }
        impl<T: $($crate::Buttons<$a, Result: $i> + )* $crate::__::Sized,$($t)*> $i<$($t)*> for T{
        }
    };
}

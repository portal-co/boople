#![no_std]
#![cfg_attr(feature = "unsize", feature(unsize))]
use core::mem::MaybeUninit;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
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
pub trait BoxButtons<B, R: ?Sized> {
    // type Result;
    fn push(self: Box<Self>, buttons: B) -> Box<R>;
}
#[cfg(feature = "alloc")]
const _: () = {
    impl<B, T: Buttons<B, Result: Unsize<R>>, R: ?Sized> BoxButtons<B, R> for T {
        fn push(self: Box<Self>, buttons: B) -> Box<R> {
            Box::new(Buttons::push(*self, buttons)) as Box<R>
        }
    }
    use core::marker::Unsize;

    impl<'a, B, T: Unsize<dyn BoxButtons<B, T> + 'a> + ?Sized + 'a> Buttons<B> for Box<T> {
        type Result = Box<T>;

        fn push(self, buttons: B) -> Self::Result {
            let this: Box<dyn BoxButtons<B, T> + 'a> = self;
            BoxButtons::push(this, buttons)
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

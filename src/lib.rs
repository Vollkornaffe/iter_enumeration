#![no_std]

//! This crate provides utility to unify [`Iterator`]s over the same type.
//!
//! Dealing with [`Iterator`] types originating from different parts of the code,
//! such as separate branches of a `match`, can be a bit cumbersome.
//! A similar problem arises with an iterator over iterators.
//!
//! Alternative solutions to this problem:
//! One can [`Iterator::collect`] the iterator, do something with `Box<dyn Iterator>`,
//! or redesign altogether.
//!
//! However, this crate allows you to do this:
//!
//! # Example
//!
//! ```rust
//! use iter_enumeration::{IterEnum2, IntoIterEnum2};
//!
//! // start with any iterator
//! let it = 0..10;
//!
//! // have some branching code
//! let it = if true {
//!     // and call the trait extension method
//!     it.map(|i| i * i).iter_enum_2a()
//! } else {
//!     // or use the enum directly
//!     IterEnum2::B(it.filter(|i| i % 2 == 0))
//! };
//!
//! // continue with the unified iterator enum type
//! let it = it.inspect(|i| println!("{i}"));
//!
//! // or have more branches, up to 6, currently
//! use iter_enumeration::IntoIterEnum3;
//! let it = match 42 {
//!     0 => it.iter_enum_3a(),
//!     1 => it.take(10).iter_enum_3b(),
//!     _ => it.skip(10).iter_enum_3c(),
//! };
//! ```
//!
//! Note that there is also [iter-enum](https://crates.io/crates/iter-enum),
//! which provides a similar solution but differs from what I want:
//! You either have to define the enum yourself or use [auto_enum](https://crates.io/crates/auto_enums),
//! which uses proc-macros.

macro_rules! impl_iter_enum {
    (
        $EnumId:ident,
        $IntoTraitId:ident,
        $((
            $A:ident,
            $iter_enum_a:ident,
            ($($Before:ident),*),($($After:ident),*)
        )),*,
    )  => {
        #[derive(Clone, Debug)]
        pub enum $EnumId<$($A),*> {
            $($A($A),)*
        }

        impl<I, $($A: Iterator<Item = I>),*> Iterator for $EnumId<$($A),*> {
            type Item = I;
            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    $(Self::$A(inner) => inner.next()),*
                }
            }
        }

        pub trait $IntoTraitId: Sized {
            $(
                fn $iter_enum_a<$($Before,)*$($After,)*>(self) -> $EnumId<$($Before,)* Self, $($After,)*> {
                    $EnumId::$A(self)
                }
            )*
        }

        impl<T: Iterator> $IntoTraitId for T {}
    };
}

impl_iter_enum!(
    IterEnum2,
    IntoIterEnum2,
    (A, iter_enum_2a, (), (B)),
    (B, iter_enum_2b, (A), ()),
);

impl_iter_enum!(
    IterEnum3,
    IntoIterEnum3,
    (A, iter_enum_3a, (), (B, C)),
    (B, iter_enum_3b, (A), (C)),
    (C, iter_enum_3c, (A, B), ()),
);

impl_iter_enum!(
    IterEnum4,
    IntoIterEnum4,
    (A, iter_enum_4a, (), (B, C, D)),
    (B, iter_enum_4b, (A), (C, D)),
    (C, iter_enum_4c, (A, B), (D)),
    (D, iter_enum_4d, (A, B, C), ()),
);

impl_iter_enum!(
    IterEnum5,
    IntoIterEnum5,
    (A, iter_enum_5a, (), (B, C, D, E)),
    (B, iter_enum_5b, (A), (C, D, E)),
    (C, iter_enum_5c, (A, B), (D, E)),
    (D, iter_enum_5d, (A, B, C), (E)),
    (E, iter_enum_5e, (A, B, C, D), ()),
);

impl_iter_enum!(
    IterEnum6,
    IntoIterEnum6,
    (A, iter_enum_6a, (), (B, C, D, E, F)),
    (B, iter_enum_6b, (A), (C, D, E, F)),
    (C, iter_enum_6c, (A, B), (D, E, F)),
    (D, iter_enum_6d, (A, B, C), (E, F)),
    (E, iter_enum_6e, (A, B, C, D), (F)),
    (F, iter_enum_6f, (A, B, C, D, F), ()),
);

#[cfg(test)]
mod tests {
    use core::iter::{empty, once};

    use super::*;

    #[test]
    fn if_branch() {
        let eval = |b| {
            if b {
                (0..10).iter_enum_2a()
            } else {
                (0..10).filter(|i| i % 2 == 0).iter_enum_2b()
            }
            .count()
        };

        assert!(eval(true) == 10);
        assert!(eval(false) == 5);
    }

    #[test]
    fn match_branch() {
        let eval = |i| {
            match i {
                0 => (0..1).iter_enum_3a(),
                1 => (0..3).iter_enum_3b(),
                _ => (0..4).iter_enum_3c(),
            }
            .count()
        };

        assert!(eval(0) == 1);
        assert!(eval(1) == 3);
        assert!(eval(42) == 4);
    }

    #[test]
    fn iterator_of_iterators() {
        assert!(
            empty()
                .chain(once((0..1).iter_enum_3a()))
                .chain(once((0..2).map(|i| i - 1).iter_enum_3b()))
                .chain(once((0..3).filter(|i| i % 2 == 0).iter_enum_3c()))
                .fold(0, |sum, it| sum + it.sum::<i32>())
                == 1
        );
    }
}

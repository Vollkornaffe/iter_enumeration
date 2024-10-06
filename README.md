# iter_enumeration

This crate provides utility to unify [`Iterator`]s over the same type.

Dealing with [`Iterator`] types originating from different parts of the code,
such as separate branches of a `match`, can be a bit cumbersome.
A similar problem arises with an iterator over iterators.

Alternative solutions to this problem:
One can [`Iterator::collect`] the iterator, do something with `Box<dyn Iterator>`,
or redesign altogether.

However, this crate allows you to do this:

# Example

```rust
use iter_enumeration::{IterEnum2, IntoIterEnum2};

// start with any iterator
let it = 0..10;

// have some branching code
let it = if true {
    // and call the trait extension method
    it.map(|i| i * i).iter_enum_2a()
} else {
    // or use the enum directly
    IterEnum2::B(it.filter(|i| i % 2 == 0))
};

// continue with the unified iterator enum type
let it = it.inspect(|i| println!("{i}"));

// or have more branches, up to 6, currently
use iter_enumeration::IntoIterEnum3;
let it = match 42 {
    0 => it.iter_enum_3a(),
    1 => it.take(10).iter_enum_3b(),
    _ => it.skip(10).iter_enum_3c(),
};
```

Note that there is also [iter-enum](https://crates.io/crates/iter-enum),
which provides a similar solution but differs from what I want:
You either have to define the enum yourself or use [auto_enum](https://crates.io/crates/auto_enums),
which uses proc-macros.

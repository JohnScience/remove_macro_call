# Attribute macro that removes a call of a function-like macro

In a vaccum, `remove_macro_call` [attribute] is fairly useless because unconditional application of `#[remove_macro_call]` [attribute] to either [declarative](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/#declarativemacrosinrust) or [procedural function-like macros][proc macro] yields the same result as writing only the code inside of the enclosing punctuation (parentheses, braces, or square brackets).

## Example

```rust
use remove_macro_call::remove_macro_call;

macro_rules! reorder_statements {
    ($s1:stmt; $s2:stmt;) => {
        $s2;
        $s1;
    };
}

let mut n = 2;
reorder_statements! {
    n += 2;
    n *= 2;
}

// 2*2+2 = 6
assert_eq!(n, 6);

// The new variable shadows the old one
let mut n = 2;

n += 2;
n *= 2;

// (2+2)*2 = 8
assert_eq!(n, 8);

// The new variable shadows the old one
let mut n = 2;
#[remove_macro_call]
reorder_statements! {
    n += 2;
    n *= 2;
}

// (2+2)*2 = 8
assert_eq!(n, 8);
```

However, with [`cfg_attr`] [attribute] `remove_macro_call` allows one to remove macro calls **conditionally**. One important application of such combination is providing support for stable toolchain while also providing functionality relying on Nightly features.

## Example

```rust, ignore
#![cfg_attr(feature = "const_trait_impl", feature(const_trait_impl))]
#![cfg_attr(feature = "const_default_impls", feature(const_default_impls))]
#![cfg_attr(feature = "const_fn_trait_bound", feature(const_fn_trait_bound))]

#[cfg(not(all(
    feature = "const_trait_impl",
    feature = "const_default_impls",
    feature = "const_fn_trait_bound"
)))]
use const_trait_impl::unconst_trait_impl;
use core::{default::Default, marker::PhantomData};
#[cfg(all(
    feature = "const_trait_impl",
    feature = "const_default_impls",
    feature = "const_fn_trait_bound"
))]
use remove_macro_call::remove_macro_call;

// Since ZST is both Eq and and PartialEq, it has structural match
// https://github.com/rust-lang/rust/issues/63438
#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, Copy)]
pub struct ZST<T: ?Sized>(PhantomData<T>);

pub trait TraitName {}

#[cfg_attr(
    all(
        feature = "const_trait_impl",
        feature = "const_default_impls",
        feature = "const_fn_trait_bound"
    ),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T: ?Sized> const TraitName for ZST<T> {}
}

// With `cargo build --features const_trait_impl, const_default_impls, const_fn_trait_bound`
// or with `cargo build --all-features, the code below is expanded as is. Otherwise,
// it gets "unconsted" to be supported by stable toolchain.
#[cfg_attr(
    all(
        feature = "const_trait_impl",
        feature = "const_default_impls",
        feature = "const_fn_trait_bound"
    ),
    remove_macro_call
)]
unconst_trait_impl! {
    impl<T: ~const TraitName + ?Sized> const Default for ZST<T> {
        fn default() -> Self {
            ZST(Default::default())
        }
    }
}
```

**Note**: In the real code, the example above could be replaced with a simpler version relying on [`cfg_aliases`](https://crates.io/crates/cfg_aliases) crate.

# Real-world examples:
* [zst](https://github.com/JohnScience/zst)

You can learn more about `const_trait_impl` here:
* [GitHub](https://github.com/JohnScience/const_trait_impl)
* [crates.io](https://crates.io/crates/const_trait_impl)

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[attribute]: https://doc.rust-lang.org/reference/attributes.html
[proc macro]: https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/#functionlikemacros
[`cfg_attr`]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute
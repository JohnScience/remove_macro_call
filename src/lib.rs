use proc_macro::TokenStream;
use syn::{parse_macro_input, Macro};

/// Unconditionally removes a call of a function-like macro
/// 
/// # Example
/// 
/// ```rust, ignore
/// #![cfg_attr(feature = "const_trait_impl", feature(const_trait_impl))]
/// #![cfg_attr(feature = "const_default_impls", feature(const_default_impls))]
/// #![cfg_attr(feature = "const_fn_trait_bound", feature(const_fn_trait_bound))]
/// 
/// #[cfg(not(all(
///     feature = "const_trait_impl",
///     feature = "const_default_impls",
///     feature = "const_fn_trait_bound"
/// )))]
/// use const_trait_impl::unconst_trait_impl;
/// use core::{default::Default, marker::PhantomData};
/// #[cfg(all(
///     feature = "const_trait_impl",
///     feature = "const_default_impls",
///     feature = "const_fn_trait_bound"
/// ))]
/// use remove_macro_call::remove_macro_call;
/// 
/// // Since ZST is both Eq and and PartialEq, it has structural match
/// // https://github.com/rust-lang/rust/issues/63438
/// #[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd, Copy)]
/// pub struct ZST<T: ?Sized>(PhantomData<T>);
/// 
/// pub trait TraitName {}
/// 
/// #[cfg_attr(
///     all(
///         feature = "const_trait_impl",
///         feature = "const_default_impls",
///         feature = "const_fn_trait_bound"
///     ),
///     remove_macro_call
/// )]
/// unconst_trait_impl! {
///     impl<T: ?Sized> const TraitName for ZST<T> {}
/// }
/// 
/// // With `cargo build --features const_trait_impl, const_default_impls, const_fn_trait_bound`
/// // or with `cargo build --all-features, the code below is expanded as is. Otherwise,
/// // it gets "unconsted" to be supported by stable toolchain.
/// #[cfg_attr(
///     all(
///         feature = "const_trait_impl",
///         feature = "const_default_impls",
///         feature = "const_fn_trait_bound"
///     ),
///     remove_macro_call
/// )]
/// unconst_trait_impl! {
///     impl<T: ~const TraitName + ?Sized> const Default for ZST<T> {
///         fn default() -> Self {
///             ZST(Default::default())
///         }
///     }
/// }
/// ```
/// 
/// **Note**: In the real code, the example above could be replaced with a simpler version relying on [`cfg_aliases`](https://crates.io/crates/cfg_aliases) crate.
/// 
/// You can learn more about `const_trait_impl` here:
/// * [GitHub](https://github.com/JohnScience/const_trait_impl)
/// * [crates.io](https://crates.io/crates/const_trait_impl)

#[proc_macro_attribute]
pub fn remove_macro_call(_args: TokenStream, item: TokenStream) -> TokenStream {
    let macro_call = parse_macro_input!(item as Macro);
    macro_call.tokens.into()
}

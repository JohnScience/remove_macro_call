use proc_macro::TokenStream;
use syn::{parse_macro_input, Macro};

#[proc_macro_attribute]
pub fn remove_macro_call(_args: TokenStream, item: TokenStream) -> TokenStream {
    let macro_call = parse_macro_input!(item as Macro);
    macro_call.tokens.into()
}

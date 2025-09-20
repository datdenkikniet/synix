use proc_macro::TokenStream;
use quote::quote;
use synix_parser::Expr;

#[proc_macro]
pub fn nix(input: proc_macro::TokenStream) -> TokenStream {
    let _ = syn::parse_macro_input!(input as Expr);

    quote! { 1 }.into()
}

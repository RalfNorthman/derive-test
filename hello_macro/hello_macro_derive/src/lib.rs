extern crate proc_macro;

use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn;
use syn_util;

#[proc_macro_derive(HelloMacro, attributes(my_custom_attribute))]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("HelloMacro syn parse failed");

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let attrs = &ast.attrs;
    let my_attr: String = syn_util::get_attribute_value(attrs, &["my_custom_attribute"])
        .expect("Attribute not found");
    let module_name = format!("__{}_mod__", my_attr);
    let module = syn::Ident::new(&module_name, Span::call_site());
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}",
                         stringify!(#name));
                println!("{}", stringify!(#my_attr));
                #module::inside_func();
            }
        }
    };
    gen.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

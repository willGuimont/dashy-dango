use proc_macro::TokenStream;

use quote::quote;
use syn;

#[proc_macro_derive(Component)]
pub fn base_component_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_base_component_macro(&ast)
}

fn impl_base_component_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl BaseComponent for #name {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
    gen.into()
}

use proc_macro::TokenStream;
use quote::quote;

use crate::base::derive_base;




/// Implementation of the [`Component`](crate::Component) proc macro
pub(crate) fn derive_component(mut ast: syn::DeriveInput) -> TokenStream {
    let base_impl = derive_base(&mut ast);

    let type_name = ast.ident;

    // ast.generics.make_where_clause().predicates.push(parse_quote! {});

    // let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    let gen = quote! {
        #base_impl
        impl Component for #type_name {
            fn get_name() -> String {
                String::from(stringify!(#type_name))
            }
        }
    };

    gen.into()
}

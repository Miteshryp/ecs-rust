use syn::DeriveInput;
use quote::quote;

use crate::base::derive_base;


pub(crate) fn derive_resource(mut ast: DeriveInput) -> proc_macro::TokenStream {
    let base_impl = derive_base(&mut ast);
    let type_name = ast.ident;

    let generate = quote! {
        #base_impl
        impl Resource for #type_name {
            fn get_name(&self) -> String {
                String::from(stringify!(#type_name))
            }
        }
    };
    generate.into()
}
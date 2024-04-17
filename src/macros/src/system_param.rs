use syn;
use quote::quote;

use crate::base::derive_base;

pub(crate) fn derive_system_param(mut ast: syn::DeriveInput) -> proc_macro::TokenStream {
    let base_impl = derive_base(&mut ast);
    // let type_name = ast.ident;

    let generate = quote! {
        #base_impl
    };

    generate.into()
}
use syn;
use quote::quote;

use crate::base::derive_base;

pub(crate) fn derive_event(mut ast: syn::DeriveInput) -> proc_macro::TokenStream {
    let base_impl = derive_base(&mut ast);
    let type_name = ast.ident;

    let generate = quote! {

        #base_impl
        impl Event for #type_name {
            fn event_type_id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<Self>()
            }
        }
    };

    generate.into()
}
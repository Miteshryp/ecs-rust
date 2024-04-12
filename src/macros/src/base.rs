use quote::quote;
use syn::parse_quote;

pub(crate) fn derive_base(ast: &mut syn::DeriveInput) -> proc_macro2::TokenStream {
    let type_name = &ast.ident;
    ast.generics
        .make_where_clause()
        .predicates
        .push(parse_quote! {
            Self: Sized + 'static
        });
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    quote! {
        impl #impl_generics ECSBase for #type_name #type_generics #where_clause {
            fn as_any(&self) -> &dyn std::any::Any {
                self as &dyn std::any::Any
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any{
                self as &mut dyn std::any::Any
            }
        }
    }
}

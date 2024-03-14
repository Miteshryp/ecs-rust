use syn::parse_quote;
use quote::quote;



pub(crate) fn derive_base(ast: &mut syn::DeriveInput) -> proc_macro2::TokenStream {
    let type_name = &ast.ident;
    ast.generics.make_where_clause().predicates.push(parse_quote! {
        Self: Sized + 'static
    });
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();
    

    quote! {
        impl #impl_generics ECSBase for #type_name #type_generics #where_clause {
            fn as_any(&self) -> &dyn Any {
                self as &dyn Any
            }
        
            fn as_any_mut(&mut self) -> &mut dyn Any{
                self as &mut dyn Any
            }
        
            fn downcast_to_ref<T: ECSBase + Sized + 'static>(&self) -> &T where Self: Sized {
                self.as_any().downcast_ref::<T>().unwrap()
            }
        
            fn downcast_to_ref_mut<T: ECSBase + Sized + 'static>(&mut self) -> &mut T where Self: Sized {
                self.as_any_mut().downcast_mut::<T>().unwrap()
            }
        }
    }
}



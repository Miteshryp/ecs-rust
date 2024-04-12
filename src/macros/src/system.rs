use crate::base::derive_base;
use quote::quote;




pub fn derive_resource_system(mut input: syn::DeriveInput) -> proc_macro::TokenStream {
    let base_derive = derive_base(&mut input);
    
    // let (impl_generic, impl_type , where_clause) = input.generics.split_for_impl();
    let type_name = input.ident;

    let generate = quote! {
        #base_derive
        impl BaseSystem for #type_name {
            fn process_update(&mut self, world_container: &mut UnsafeWorldContainer) {
                let resource_id = world_container.get_world().get_resource_id::< <Self as ResourceSystem>::ResourceType>();
                self.on_update(&mut world_container.get_world_mut(), );
            }

            fn process_start(&mut self, world_container: &mut UnsafeWorldContainer) {

            }

            fn process_events(&mut self, world_container: &mut UnsafeWorldContainer) {

            }
        }
    };

    generate.into()
}

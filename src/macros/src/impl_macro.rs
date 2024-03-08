use proc_macro::TokenStream;
use quote::quote;
use syn;


pub(crate) fn component_system_impl(ast: syn::DeriveInput) -> TokenStream {
    let type_name = ast.ident;

    let generate = quote! {
        impl BaseSystem for #type_name {
    
            fn as_any(&self) -> &dyn Any {
                self as &dyn Any
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self as &mut dyn Any
            }
            
            fn process_update(&mut self, world: Rc<RefCell<World>>) {
                println!("Here");
                let system = self.as_any_mut().downcast_mut::<#type_name>().unwrap();
                
                let mut binding = world.as_ref().borrow_mut();
                let component_list = binding.get_all_components_mut();
        
                for (component, entity_id) in component_list {
                    system.on_update(world.clone(), *entity_id, component);
                }
            }

            fn process_start(&mut self, world: Rc<RefCell<World>>) {
                println!("Again Here");
                let system = self.as_any_mut().downcast_mut::<#type_name>().unwrap();
                system.on_start(world.clone());
            }
        
        }
    };

    generate.into()
}


pub(crate) fn component_impl(ast: syn::DeriveInput) -> TokenStream {
    let type_name = ast.ident;
    
    let gen = quote! {
        impl Component for #type_name {
            fn get_name() -> String {
                String::from(stringify!(#type_name))
            }

            fn as_any(&self) -> &dyn Any {
                self as &dyn Any
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self as &mut dyn Any
            }
        }
    };

    gen.into()
}
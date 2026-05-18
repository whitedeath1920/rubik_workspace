extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Ops)]
pub fn derive_ops(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_ops(&input).into()
}

fn impl_ops(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => panic!("Ops solo soporta structs"),
    };

    let (field_names,field_types): (Vec<_>, Vec<_>)= fields
        .iter()
        .map(|f| (f.ident.as_ref().unwrap(),&f.ty))
        .unzip();

    let new_impl = quote! {
        impl #name {
            #[inline(always)]
            pub fn new() -> Self {
                Self {
                    #(
                      #field_names: <#field_types as Default>::default(),
                    )*
                }
            }
        }
    };
    let display_impl = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_> ) -> core::fmt::Result {
                #(
                    writeln!(f,"{},",self.#field_names)?;
                )*
                Ok(())
            }
        }
    };
    let ptr_impl = quote! {
        impl #name {
            #[inline(always)]
            fn ptr() -> std::ptr::NonNull<u8> {
                unsafe {
                    let layout = std::alloc::Layout::new::<#name>();
                    
                    let raw = alloc(layout);
                    if raw.is_null() {
                        std::alloc::handle_alloc_error(layout);
                    }
                    
                    let typed = raw as *mut #name;
                    core::ptr::write(typed,#name::new());
                    
                    std::ptr::NonNull::new_unchecked(raw as *mut u8)
                }
            }
        }
    };
    let add_impl = quote! {
        impl #name {
            pub fn add(mut self,lhs:&#name) -> Self {
                #(
                    self.#field_names.add_assign(&lhs.#field_names);
                )*
                self
            }
        }
    };
    let ops_name = format_ident!("OPS_{}",name.to_string().to_uppercase());
    let ops_cube = quote! {
        pub static #ops_name: CubeOps = CubeOps {
           to_string: |ptr| {
               let cube: &#name = unsafe { &*(ptr as *const #name) };
               cube.to_string()
           },
           // add: |a, b| {
           //     let mut cube_a: &#name = unsafe { &*(a as *mut #name) }; 
           //     let cube_b: &#name = unsafe { &*(b as *const #name) }; 
           //     cube_a.add(cube_b);
           // }
        };
    };
    quote! {
        #new_impl
        #display_impl
        #add_impl
        #ptr_impl
        #ops_cube
    }
}

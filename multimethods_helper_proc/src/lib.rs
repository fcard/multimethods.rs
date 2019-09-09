extern crate proc_macro;

use proc_macro as pm;
use proc_macro2 as pm2;
use syn::*;
use quote::*;

#[proc_macro]
pub fn impl_types(tokens: pm::TokenStream) -> pm::TokenStream {
  let n: usize = tokens.to_string().parse().expect("impl_types! expects a number");

  let mut type_parameters = Vec::new();
  let mut where_clauses   = Vec::new();
  let mut type_ids        = Vec::new();
  let mut is_refs         = Vec::new();
  let mut type_n_def      = Vec::new();

  let types = quote!(crate::types);

  for i in 0..n {
    let ix = syn::Index::from(i);
    let tp = Ident::new(&format!("T{}", i), pm2::Span::call_site());

    type_parameters.push(tp.clone());
    where_clauses.push(quote!(#tp: #types::SubType));
    type_ids.push(quote!(self.#ix.type_of()));
    is_refs.push(quote!(self.#ix.is_ref()));

    let type_n = Ident::new(&format!("type{}", i), pm2::Span::call_site());
    type_n_def.push(quote! {
      fn #type_n(&self) -> #types::ConcreteType {
        #types::ConcreteType {
          id:     self.#ix.type_of(),
          is_ref: self.#ix.is_ref(),
          parent: self.#ix.abstract_type()
        }
      }
    });
  }

  for i in n..12 {
    type_ids.push(quote!(::std::any::TypeId::of::<!>()));
    is_refs.push(quote!(false));

    let type_n = Ident::new(&format!("type{}", i), pm2::Span::call_site());
    type_n_def.push(quote! {
      fn #type_n(&self) -> #types::ConcreteType {
        #types::ConcreteType {
          id:     ::std::any::TypeId::of::<!>(),
          is_ref: false,
          parent: #types::TOP
        }
      }
    });
  }

  quote!(
    impl<#(#type_parameters),*> #types::Types for (#(#type_parameters,)*)
      where
        #(#where_clauses,)*
    {
      fn type_tuple(&self) -> #types::TypeTuple {
        (#(#type_ids),*)
      }

      fn has_ref(&self) -> bool {
        false #(|| #is_refs)*
      }

      #(#type_n_def)*
    }
  ).into()
}

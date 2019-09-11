#![feature(decl_macro)]
extern crate proc_macro;

use proc_macro as pm;
use proc_macro2 as pm2;
use syn::*;
use quote::*;

macro ident($str: literal$(, $expr: expr)*) {
  Ident::new(&format!($str$(, $expr)*), pm2::Span::call_site())
}

const MAX_ARGS: usize = 12;

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
    let tp = ident!("T{}", i);

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

  for i in n..MAX_ARGS {
    is_refs.push(quote!(false));

    let type_n = Ident::new(&format!("type{}", i), pm2::Span::call_site());
    type_n_def.push(quote! {
      fn #type_n(&self) -> #types::ConcreteType {
        #types::ConcreteType {
          id:     ::std::any::TypeId::of::<crate::value::Value>(),
          is_ref: false,
          parent: #types::ANY
        }
      }
    });
  }

  let variant = Ident::new(&format!("T{}", n), pm2::Span::call_site());

  quote!(
    impl<#(#type_parameters),*> #types::Types for (#(#type_parameters,)*)
      where
        #(#where_clauses,)*
    {
      fn type_tuple(&self) -> #types::TypeIds {
        #types::TypeIds::#variant(#(#type_ids),*)
      }

      fn has_ref(&self) -> bool {
        false #(|| #is_refs)*
      }

      #(#type_n_def)*
    }
  ).into()
}

struct VarargArgs {
  e: Expr,
  vararg: Ident,
  positionals: Vec<Ident>
}

impl syn::parse::Parse for VarargArgs {
  fn parse(stream: syn::parse::ParseStream) -> Result<Self> {
    let e = stream.parse::<Expr>().unwrap();
    stream.parse::<Token![,]>().unwrap();
    let names = stream.parse_terminated::<Ident, Token![,]>(Ident::parse).unwrap();

    let len = names.len();
    let vararg = names.iter().last().unwrap().clone();
    let positionals = names.into_iter().take(len-1).collect();

    Ok(VarargArgs { e, vararg, positionals })
  }
}

#[proc_macro]
pub fn match_vararg(tokens: pm::TokenStream) -> pm::TokenStream {
  let VarargArgs { e, vararg, positionals } = parse_macro_input!(tokens as VarargArgs);

  let tms = quote!(crate::types::TypeMatches);
  let mut match_arms = Vec::new();

  for i in positionals.len()..=MAX_ARGS {
    let bs = (0..i).map(|j| ident!("b{}", j)).collect::<Vec<_>>();

    // normal match
    let variant = ident!("T{}", i);
    let mut matches = Vec::new();

    for j in 0..positionals.len() {
      let a = positionals[j].clone();
      let b = bs[j].clone();

      matches.push(quote! {
        #a.is_super_match(&#b)
      });
    }

    for j in positionals.len()..i {
      let b = bs[j].clone();

      matches.push(quote! {
        #vararg.is_super_match(&#b)
      });
    }

    match_arms.push(quote! {
      #tms::#variant(#(#bs),*) => true #(&& #matches)*,
    });


    // vararg match
    if i < MAX_ARGS {
      let variant = ident!("V{}", i+1);
      matches.push(quote! {
        #vararg.is_super_match(bv)
      });

      match_arms.push(quote! {
        #tms::#variant(#(#bs,)* bv) => true #(&& #matches)*,
      });
    }
  }

  if positionals.len() > 0 {
    match_arms.push(quote! {
      _ => false
    });
  }

  quote!(
    match #e {
      #(#match_arms)*
    }
  ).into()
}

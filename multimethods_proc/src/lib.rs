extern crate proc_macro;
use std::collections::HashMap;

use proc_macro as pm;
use proc_macro2 as pm2;
use syn::*;
use syn::punctuated::Punctuated;
use syn::token::{Paren, Comma};
use quote::*;

struct Method {
  public: bool,
  expr: pm2::TokenStream,
}

type Methods = HashMap<Ident, Vec<Method>>;

struct Keys(Vec<Ident>);

impl syn::parse::Parse for Keys {
  fn parse(input: syn::parse::ParseStream) -> Result<Self> {
    let punct = <Punctuated<Ident, Comma>>::parse_terminated(input).unwrap();
    Ok(Keys(punct.into_iter().collect()))
  }
}

struct Key(Ident);

impl syn::parse::Parse for Key {
  fn parse(input: syn::parse::ParseStream) -> Result<Self> {
    let key_token: Ident = input.parse().unwrap();
    assert_eq!(key_token.to_string(), "key".to_string());

    let _: Token![=] = input.parse().unwrap();

    let key_name: Ident = input.parse().unwrap();
    let _: Token![;] = input.parse().unwrap();

    Ok(Key(key_name))
  }
}

struct MethodDecls(Key, Vec<ItemFn>);

impl syn::parse::Parse for MethodDecls {
  fn parse(input: syn::parse::ParseStream) -> Result<Self> {
    let key = input.parse().unwrap();
    let mut decls = Vec::new();
    while !input.is_empty() {
      if let Ok(meth) = input.parse() {
        decls.push(meth)
      }
    }
    Ok(MethodDecls(key, decls))
  }
}

enum NewGenericFunction {
  JustName(Ident),
  Methods(Vec<ItemFn>)
}

impl syn::parse::Parse for NewGenericFunction {
  fn parse(input: syn::parse::ParseStream) -> Result<Self> {
    if input.peek(Ident) {
      Ok(NewGenericFunction::JustName(input.parse().unwrap()))

    } else {
      let mut decls = Vec::new();
      while !input.is_empty() {
        if let Ok(meth) = input.parse() {
          decls.push(meth)
        }
      }
      Ok(NewGenericFunction::Methods(decls))
    }
  }
}


#[proc_macro_attribute]
pub fn __fmc(_: pm::TokenStream, tokens: pm::TokenStream) -> pm::TokenStream {
  let mut iter = tokens.into_iter();

  let ident = iter.next().unwrap();
  let punct = iter.next().unwrap();
  let group = iter.next().unwrap();

  match (ident, punct, group) {
    (pm::TokenTree::Ident(ident),
     pm::TokenTree::Punct(punct),
     pm::TokenTree::Group(group)) => {
      assert_eq!(punct.to_string(), "!".to_string());

      if ident.to_string() == "multimethods".to_string() {
        multimethods_impl(group.stream(), true)
      } else {
        multifunction_impl(group.stream(), true)
      }
    }
    _ => panic!("")
  }
}

#[proc_macro]
pub fn multifunction(tokens: pm::TokenStream) -> pm::TokenStream {
  multifunction_impl(tokens, false)
}

fn multifunction_impl(tokens: pm::TokenStream, fmc: bool) -> pm::TokenStream {
  let new = parse_macro_input!(tokens as NewGenericFunction);
  let lazy_static_crate = lazy_static_crate(fmc);
  let root = root(fmc);
  (match new {
    NewGenericFunction::JustName(name) => {
      quote! {
        #lazy_static_crate::lazy_static! {
          static ref #name: #root GenericFunction = {
            (#root GENERIC_FUNCTIONS).new_function(#root MethodTable::new())
          };
        }
      }
    }

    NewGenericFunction::Methods(decls) => {
      let methods = method_defs(decls.iter(), fmc);

      let mut name = None;
      let mut defs = None;

      for (func, meths) in methods {
        if name.is_some() {
          panic!("multifunction! can only define one function at a time");
        } else {
          name = Some(func);
        }

        defs = Some(meths);
      }

      let name  = name.unwrap();
      let defs  = defs.unwrap();
      let exprs = defs.iter().map(|a| a.expr.clone());
      let vis   = get_vis(is_public(defs.iter().map(|a| a.public)));

      quote! {
        #lazy_static_crate::lazy_static! {
          #vis static ref #name: #root GenericFunction = {
            let mut table = #root MethodTable::new();
            #(#exprs;)*
            (#root GENERIC_FUNCTIONS).new_function(table)
          };
        }
      }
    }
  }).into()
}

#[proc_macro]
pub fn multimethods(tokens: pm::TokenStream) -> pm::TokenStream {
  multimethods_impl(tokens, false)
}

fn multimethods_impl(tokens: pm::TokenStream, fmc: bool) -> pm::TokenStream {
  let decls = parse_macro_input!(tokens as MethodDecls);
  let key = (decls.0).0;
  let methods = method_defs(decls.1.iter(), fmc);

  let root = root(fmc);
  let mut defs  = Vec::new();
  let mut inits = Vec::new();
  let lazy_static_crate = lazy_static_crate(fmc);
  let mut public = None;

  for (func, meths) in methods {
    inits.push(
      quote! {
        #lazy_static_crate::initialize(&#func);
      }
    );

    let exprs  = meths.iter().map(|a| a.expr.clone());

    if let Some(p) = public {
       assert_eq!(p, is_public(meths.iter().map(|a| a.public)));
     } else {
       public = Some(is_public(meths.iter().map(|a| a.public)));
     }

    defs.push(
      quote! {
        {
          let mut table = functions.get_mut(&#func).unwrap();
          #(#exprs;)*
        }
      }
    );
  }

  let vis = get_vis(public.unwrap());

  quote!(
    #lazy_static_crate::lazy_static! {
      #vis static ref #key: #root method::MethodKey = {
        #(#inits)*
        (#root GENERIC_FUNCTIONS).with_functions_mut(|functions| {
          #(#defs)*
        });
        MethodKey
      };
    }
  ).into()
}

fn method_defs<'a, I: Iterator<Item=&'a ItemFn>>(item_fns: I, fmc: bool) -> Methods {
  let mut methods = Methods::new();

  for item_fn in item_fns {
    let name        = item_fn.sig.ident.clone();
    let num_args    = item_fn.sig.inputs.len();
    let is_abstract = has_abstract_type(args(&item_fn.sig));
    let types       = types(args(&item_fn.sig), &item_fn.sig.output, is_abstract, fmc);
    let closure     = create_closure(&item_fn);
    let insertion   = get_insertion_function(is_abstract);
    let variant     = get_variant(num_args, &item_fn.sig.output);
    let inner_func  = get_inner_function(num_args, &item_fn.sig.output);
    let constructor = get_inner_constructor(args(&item_fn.sig), &item_fn.sig.output);
    let inner_trait = get_inner_trait(args(&item_fn.sig), &item_fn.sig.output, fmc);


    if !methods.contains_key(&name) {
      methods.insert(name.clone(), Vec::new());
    }

    let of_func = methods.get_mut(&name).unwrap();

    of_func.push(
      Method {
        public: if let Visibility::Public(_) = item_fn.vis { true } else { false },

        expr: if num_args == 0 {
          quote! {
            table.#insertion(
              #types,
              Function::#variant(#inner_func::new(#closure))
            )
          }
        } else {
          quote! {
            table.#insertion(
              #types,
              Function::#variant(<#inner_func as #inner_trait>::#constructor(#closure))
            )
          }
        }
      }
    );
  }
  methods
}

fn is_public<I: Iterator<Item=bool>>(vis: I) -> bool {
  let mut public = None;

  for v in vis {
    if let Some(p) = &public {
      assert_eq!(p, &v);
    } else {
      public = Some(v);
    }
  }
  public.unwrap()
}

fn get_vis(b: bool) -> pm2::TokenStream {
  if b {
    quote!(pub)
  } else {
    quote!()
  }
}

use std::fmt;
#[allow(dead_code)]
fn deb<T: fmt::Display>(x: T) -> T {
  println!("{}", x);
  x
}

fn lazy_static_crate(fmc: bool) -> pm2::TokenStream {
  if fmc {
    quote!(::lazy_static)
  } else {
    quote!(::multimethods::lazy_static)
  }
}

fn args(sig: &Signature) -> impl Iterator<Item=FnArg> {
  sig.inputs.clone().into_iter()
}


fn types<I>(inputs: I, output: &ReturnType, is_abs: bool, fmc: bool) -> pm2::TokenStream
  where
    I: Iterator<Item=FnArg>
{
  if is_abs {
    type_matches(inputs, output, fmc)

  } else {
    type_ids(inputs, output)
  }
}


fn type_ids<I>(inputs: I, output: &ReturnType) -> pm2::TokenStream
  where
    I: Iterator<Item=FnArg>
{
  let mut types      = Vec::new();
  let mut ref_inputs = false;

  for input in inputs {
    let ty = arg_type(input);
    ref_inputs = ref_inputs || is_ref(&ty);
    types.push(quote!(<#ty>::associated_type_of()));
  }

  while types.len() < 12 {
    types.push(quote!(::std::any::TypeId::of::<!>()));
  }

  let returns_ref = is_ref_return(output);

  quote! {
    (((#(#types,)*), #ref_inputs), #returns_ref)
  }
}


fn type_matches<I>(inputs: I, output: &ReturnType, fmc: bool) -> pm2::TokenStream
  where
    I: Iterator<Item=FnArg>
{
  let root       = root(fmc);
  let type_match = quote!(#root types::TypeMatch);
  let sub_type   = quote!(#root types::SubType);
  let assoc_type = quote!(associated_concrete_type);
  let mut types  = Vec::new();

  for input in inputs {
    let ty = arg_type(input);
    if let Some(aty) = abstract_type(&ty) {
      types.push(quote!(#type_match::Abstract(#aty)));
    } else {
      types.push(quote!(#type_match::Concrete(<#ty as #sub_type>::#assoc_type())));
    }
  }

  while types.len() < 12 {
    types.push(quote!(#type_match::Concrete(<! as #sub_type>::#assoc_type())));
  }

  let returns_ref = is_ref_return(output);

  quote! {
    ((#(#types,)*), #returns_ref)
  }
}


fn create_closure(item: &ItemFn) -> pm2::TokenStream {
  let args     = args(&item.sig);
  let output   = &item.sig.output;
  let outty    = return_type(output);
  let body     = &item.block;
  let ref_ret  = is_ref_return(&output);
  let into_val = if ref_ret { quote!(into_value_ref) } else { quote!(into_value) };


  quote! {
    |#(#args),*| {
      let __ReturnValue_MultiMethods: #outty = #body;
      __ReturnValue_MultiMethods.#into_val()
    }
  }
}

fn return_type(ty: &ReturnType) -> pm2::TokenStream {
  match ty {
    ReturnType::Default => quote!(()),
    ReturnType::Type(_, ty) => quote!(#ty)
  }
}


fn get_insertion_function(is_abstract: bool) -> pm2::TokenStream {
  if is_abstract {
    quote!(insert_abstract)

  } else {
    quote!(insert)
  }
}


fn get_variant(n: usize, output: &ReturnType) -> pm2::TokenStream {
  let name = format!("F{}{}", n, ref_return_str(output));
  let variant = Ident::new(&name, pm2::Span::call_site());
  quote!(#variant)
}

fn get_inner_function(n: usize, output: &ReturnType) -> pm2::TokenStream {
  let name = format!("Function{}{}", n, ref_return_str(output));
  let function = Ident::new(&name, pm2::Span::call_site());
  quote!(#function)
}


fn get_inner_constructor<I>(inputs: I, output: &ReturnType) -> pm2::TokenStream
  where
    I: Iterator<Item=FnArg>
{
  if is_ref_return(output) {
    quote!(new)

  } else {
    let ref_inputs = inputs.map(arg_type).any(|t| is_ref(&t));
    if ref_inputs {
      quote!(new_r)
    } else {
      quote!(new_s)
    }
  }
}

fn get_inner_trait<I>(inputs: I, output: &ReturnType, fmc: bool) -> pm2::TokenStream
  where
    I: Iterator<Item=FnArg>
{
  let root = root(fmc);

  if is_ref_return(output) {
    quote!(#root function::inner_function::InnerFunctionRefReturnNew<_,_>)

  } else {
    let ref_inputs = inputs.map(arg_type).any(|t| is_ref(&t));
    if ref_inputs {
      quote!(#root function::inner_function::InnerFunctionRefNew<_,_>)
    } else {
      quote!(#root function::inner_function::InnerFunctionStaticNew<_,_>)
    }
  }
}

fn has_abstract_type<I>(inputs: I) -> bool
  where
    I: Iterator<Item=FnArg>
{
  for input in inputs {
    if abstract_type(&arg_type(input)).is_some() {
      return true;
    }
  }
  false
}

fn root(fmc: bool) -> pm2::TokenStream {
  if fmc {
    quote!()
  } else {
    quote!(::multimethods::)
  }
}

fn arg_type(arg: FnArg) -> Type {
  if let FnArg::Typed(pat) = arg {
    *pat.ty

  } else {
    Type::Tuple(TypeTuple {
      elems: Punctuated::new(),
      paren_token: Paren { span: pm2::Span::call_site() }
    })
  }
}

fn is_ref_return(ty: &ReturnType) -> bool {
  match ty {
    ReturnType::Default => false,
    ReturnType::Type(_, ty) => is_ref(&*ty)
  }
}

fn ref_return_str(ty: &ReturnType) -> &'static str {
  if is_ref_return(ty) {
    "R"
  } else {
    ""
  }
}

fn is_ref(ty: &Type) -> bool {
  match ty {
    Type::Paren(t) => is_ref(&*t.elem),

    Type::Verbatim(_) | Type::Infer(_) => {
      panic!("Could not determine if the type is a reference")
    }

    Type::Macro(m) => {
      if path_ends_with(&m.mac.path, "Abstract") {
        false
      } else {
        panic!("Could not determine if the type is a reference")
      }
    }

    Type::Reference(r) => {
      if let Some(lifetime) = &r.lifetime {
        lifetime.to_string() != "'static".to_string()
      } else {
        true
      }
    }

    _ => false
  }
}

fn abstract_type(ty: &Type) -> Option<Ident> {
  match ty {
    Type::Paren(t) => abstract_type(&*t.elem),

    Type::Macro(m) => {
      if path_ends_with(&m.mac.path, "Abstract") {
        let tokens = m.mac.tokens.clone();
        parse2::<Ident>(tokens).ok()

      } else {
        None
      }
    }
    _ => None

  }
}

fn path_ends_with(p: &Path, s: &str) -> bool {
  if let Some(segment) = p.segments.iter().last() {
    segment.ident.to_string() == s.to_string()

  } else {
    false
  }
}


#[allow(dead_code)]
fn arg_has_attr(f: &FnArg, attr: &str) -> bool {
  match f {
    FnArg::Typed(p) => {
      p.attrs.iter().any(|a| {
        match a.path.get_ident() {
          Some(i) => i.to_string() == attr.to_string(),
          None => false
        }
      })
    }
    _ => false
  }
}

#[allow(dead_code)]
fn has_ref_attr(f: &FnArg) -> bool {
  arg_has_attr(f, "ref")
}

#[allow(dead_code)]
fn has_abs_attr(f: &FnArg) -> bool {
  arg_has_attr(f, "abs")
}


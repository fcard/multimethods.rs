use crate::value::*;
use crate::value_ref::*;
use crate::function::helper_macros::*;

// Types

pub enum Function1 {
  S(Box<dyn Fn(Value) -> Value>),
  R(Box<dyn for<'a> Fn(ValueRef<'a>) -> Value>),
}

pub struct Function1R {
  pub inner: Box<dyn for<'a> Fn(ValueRef<'a>) -> ValueRef<'a>>
}

// Impls

impl_function! {
  type = Function1;

  constructors = [ new_s, new_r ];
  parameters = [a: &'a A];
  static_calls = [ variant=S; call_once_s, call_mut_s, call_s ];
  ref_calls = [
    variant=R;
    call_once_r,  call_mut_r,  call_r,
    call_once_rr, call_mut_rr, call_rr
  ];
}

impl_ref_function! {
  type = Function1R;

  constructor = new;
  parameters  = [a: A];
  calls = [call_once, call_mut, call]
}


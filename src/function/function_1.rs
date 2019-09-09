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
  Function1(a: A)
}

impl_ref_function! {
  Function1R(a: A)
}


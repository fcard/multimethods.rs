#![allow(unused_imports)]

use std::any::*;
use std::rc::{Rc};
use std::fmt::Debug;
use crate::value::*;

pub trait VRef<'a> = Any + 'a;

#[derive(Clone, Copy)]
pub struct ValueRef<'a> {
  pub inner: &'a (dyn Any)
}

pub trait FromValueRef<'a> {
  fn from_value_ref(v: ValueRef<'a>) -> &'a Self;
}

impl<'a, T> FromValueRef<'a> for T
  where
    T: 'static
{
  default fn from_value_ref(v: ValueRef<'a>) -> &'a Self {
    v.inner.downcast_ref::<Self>().unwrap()
  }
}

pub trait IntoValueRef<'a> {
  fn into_value_ref(&'a self) -> ValueRef<'a>;
}


impl<'a, T: 'static> IntoValueRef<'a> for T {
  default fn into_value_ref(&'a self) -> ValueRef<'a> {
    ValueRef { inner: self }
  }
}

impl<'a> IntoValueRef<'a> for ValueRef<'a> {
  default fn into_value_ref(&'a self) -> ValueRef<'a> {
    ValueRef { inner: self.inner }
  }
}

impl<'a> IntoValueRef<'a> for Value {
  default fn into_value_ref(&'a self) -> ValueRef<'a> {
    ValueRef { inner: &*self.inner }
  }
}

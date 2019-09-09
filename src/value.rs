#![allow(unused_imports)]

use std::any::*;
use std::rc::{Rc};
use std::fmt::Debug;

pub trait ValueTrait = Any + 'static;
pub trait VT         = ValueTrait;

pub struct Value {
  pub inner: Box<dyn ValueTrait>
}

pub trait FromValue: Sized {
  fn from_value(v: Value) -> Self;
}

impl FromValue for Value {
  default fn from_value(v: Value) -> Self {
    v
  }
}

impl<T: 'static> FromValue for T {
  default fn from_value(v: Value) -> Self {
    *v.inner.downcast::<Self>().unwrap()
  }
}

pub trait IntoValue {
  fn into_value(self) -> Value;
}


impl<T: 'static> IntoValue for T {
  default fn into_value(self) -> Value {
    Value { inner: box self }
  }
}

impl IntoValue for Value {
  default fn into_value(self) -> Value {
    self
  }
}


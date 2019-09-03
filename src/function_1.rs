use crate::value::*;
use crate::value_ref::*;

pub enum Function1 {
  S(Box<dyn Fn(Value) -> Value>),
  R(Box<dyn for<'a> Fn(ValueRef<'a>) -> Value>),
}

pub struct Function1R {
  inner: Box<dyn for<'a> Fn(ValueRef<'a>) -> ValueRef<'a>>
}

impl Function1 {
  pub fn s<A,R,F>(func: F) -> Self
    where
      A: FromValue,
      R: IntoValue,
      F: Fn(A) -> R + 'static
  {
    Function1::S(
      box move |av| {
        let a = A::from_value(av);
        func(a).into_value()
      }
    )
  }

  pub fn r<A,R,F>(func: F) -> Self
    where
      A: for<'a> FromValueRef<'a>,
      R: IntoValue,
      F: for<'a> Fn(&'a A) -> R + 'static
  {
    Function1::R(
      box move |av| {
        let a = A::from_value_ref(av);
        func(a).into_value()
      }
    )
  }
}

impl Function1R {
  pub fn new<A,R,F>(func: F) -> Self
    where
      A: for<'a> FromValueRef<'a> + 'static,
      R: for<'a> IntoValueRef<'a> + 'static,
      F: for<'a> Fn(&'a A) -> &'a R + 'static
  {
    Function1R {
      inner: box move |av| {
        let a = A::from_value_ref(av);
        func(a).into_value_ref()
      }
    }
  }
}

impl<A: 'static> FnOnce<(A,)> for Function1
  where
    A: IntoValue,
{
  type Output = Value;

  default extern "rust-call" fn call_once(self, a: (A,)) -> Value {
    match self {
      Function1::S(func) => {
        let av = a.0.into_value();
        func.call_once((av,))
      }
      _ => unreachable!()
    }
  }
}

impl<A> FnOnce<(&A,)> for Function1
  where
    A: for<'a> IntoValueRef<'a>
{
  extern "rust-call" fn call_once(self, a: (&A,)) -> Value {
    match self {
      Function1::R(func) => {
        let av = a.0.into_value_ref();
        func.call_once((av,))
      }
      _ => unreachable!()
    }
  }
}

impl<'a> FnOnce<(ValueRef<'a>,)> for Function1 {
  extern "rust-call" fn call_once(self, (a,): (ValueRef<'a>,)) -> Value {
    match self {
      Function1::R(func) => {
        func.call_once((a,))
      }
      _ => unreachable!()
    }
  }
}

impl<'a,A> FnOnce<(&'a A,)> for Function1R
  where
    A: IntoValueRef<'a>,
{
  type Output = ValueRef<'a>;

  extern "rust-call" fn call_once(self, a: (&'a A,)) -> ValueRef<'a> {
    let av = a.0.into_value_ref();
    self.inner.call_once((av,))
  }
}


impl<A: 'static> FnMut<(A,)> for Function1
  where
    A: IntoValue,
{
  default extern "rust-call" fn call_mut(&mut self, a: (A,)) -> Value {
    match self {
      Function1::S(func) => {
        let av = a.0.into_value();
        func.call_once((av,))
      }
      _ => unreachable!()
    }
  }
}

impl<A> FnMut<(&A,)> for Function1
  where
    A: for<'a> IntoValueRef<'a>
{
  extern "rust-call" fn call_mut(&mut self, a: (&A,)) -> Value {
    match self {
      Function1::R(func) => {
        let av = a.0.into_value_ref();
        func.call_once((av,))
      }
      _ => unreachable!()
    }
  }
}

impl<'a> FnMut<(ValueRef<'a>,)> for Function1 {
  extern "rust-call" fn call_mut(&mut self, (a,): (ValueRef<'a>,)) -> Value {
    match self {
      Function1::R(func) => {
        func.call_once((a,))
      }
      _ => unreachable!()
    }
  }
}

impl<'a,A> FnMut<(&'a A,)> for Function1R
  where
    A: IntoValueRef<'a>
{
  extern "rust-call" fn call_mut(&mut self, a: (&'a A,)) -> ValueRef<'a> {
    let av = a.0.into_value_ref();
    self.inner.call_mut((av,))
  }
}

impl<A: 'static> Fn<(A,)> for Function1
  where
    A: IntoValue
{
  default extern "rust-call" fn call(&self, a: (A,)) -> Value {
    match self {
      Function1::S(func) => {
        let av = a.0.into_value();
        func.call((av,))
      }
      _ => unreachable!()
    }
  }
}

impl<A> Fn<(&A,)> for Function1
  where
    A: for<'a> IntoValueRef<'a>
{
  extern "rust-call" fn call(&self, a: (&A,)) -> Value {
    match self {
      Function1::R(func) => {
        let av = a.0.into_value_ref();
        func.call((av,))
      }
      _ => unreachable!()
    }
  }
}

impl<'a> Fn<(ValueRef<'a>,)> for Function1 {
  extern "rust-call" fn call(&self, (a,): (ValueRef<'a>,)) -> Value {
    match self {
      Function1::R(func) => {
        func.call((a,))
      }
      _ => unreachable!()
    }
  }
}

impl<'a,A> Fn<(&'a A,)> for Function1R
  where
    A: IntoValueRef<'a>
{
  extern "rust-call" fn call(&self, a: (&'a A,)) -> ValueRef<'a> {
    let av = a.0.into_value_ref();
    self.inner.call((av,))
  }
}


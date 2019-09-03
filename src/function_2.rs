use crate::value::*;
use crate::value_ref::*;

type R<'a> = ValueRef<'a>;

pub enum Function2 {
  S(Box<dyn Fn(Value,Value) -> Value>),
  R(Box<dyn for<'a,'b> Fn(ValueRef<'a>,ValueRef<'b>) -> Value>),
}

pub struct Function2R {
  inner: Box<dyn for<'a> Fn(ValueRef<'a>, ValueRef<'a>) -> ValueRef<'a>>
}

impl Function2 {
  pub fn s<A,B,R,F>(func: F) -> Self
    where
      A: FromValue,
      B: FromValue,
      R: IntoValue,
      F: Fn(A,B) -> R + 'static
  {
    Function2::S(
      box move |av,bv| {
        let a = A::from_value(av);
        let b = B::from_value(bv);
        func(a,b).into_value()
      }
    )
  }

  pub fn r<A,B,R,F>(func: F) -> Self
    where
      A: for<'a> FromValueRef<'a>,
      B: for<'b> FromValueRef<'b>,
      R: IntoValue,
      F: for<'a,'b> Fn(&'a A, &'b B) -> R + 'static
  {
    Function2::R(
      box move |av,bv| {
        let a = A::from_value_ref(av);
        let b = B::from_value_ref(bv);
        func(a,b).into_value()
      }
    )
  }
}

impl Function2R {
  pub fn new<A,B,R,F>(func: F) -> Self
    where
      A: for<'a> FromValueRef<'a> + 'static,
      B: for<'b> FromValueRef<'b> + 'static,
      R: for<'a> IntoValueRef<'a> + 'static,
      F: for<'a,'b> Fn(&'a A, &'a B) -> &'a R + 'static
  {
    Function2R {
      inner: box move |av,bv| {
        let a = A::from_value_ref(av);
        let b = B::from_value_ref(bv);
        func(a,b).into_value_ref()
      }
    }
  }
}


// FnOnce


impl<A,B> FnOnce<(A,B)> for Function2
  where
    A: IntoValue,
    B: IntoValue,
{
  type Output = Value;

  default extern "rust-call" fn call_once(self, a: (A,B)) -> Value {
    match self {
      Function2::S(func) => {
        let av = a.0.into_value();
        let bv = a.1.into_value();
        func.call_once((av,bv))
      }
      _ => unreachable!()
    }
  }
}

impl<A,B> FnOnce<(&A,&B)> for Function2
  where
    A: for<'a> IntoValueRef<'a>,
    B: for<'b> IntoValueRef<'b>,
{
  default extern "rust-call" fn call_once(self, a: (&A,&B)) -> Value {
    match self {
      Function2::R(func) => {
        let av = a.0.into_value_ref();
        let bv = a.1.into_value_ref();
        func.call_once((av,bv))
      }
      _ => unreachable!()
    }
  }
}

impl<'a,'b> FnOnce<(ValueRef<'a>,ValueRef<'b>)> for Function2 {
  default extern "rust-call" fn call_once(self, (a,b): (R<'a>, R<'b>)) -> Value {
    match self {
      Function2::R(func) => {
        func.call_once((a,b))
      }
      _ => unreachable!()
    }
  }
}

impl<'a,A,B> FnOnce<(&'a A, &'a B)> for Function2R
  where
    A: IntoValueRef<'a>,
    B: IntoValueRef<'a>,
{
  type Output = ValueRef<'a>;

  default extern "rust-call" fn call_once(self, a: (&'a A, &'a B)) -> ValueRef<'a> {
    let av = a.0.into_value_ref();
    let bv = a.1.into_value_ref();
    self.inner.call_once((av,bv))
  }
}

// FnMut


impl<A,B> FnMut<(A,B)> for Function2
  where
    A: IntoValue,
    B: IntoValue,
{
  default extern "rust-call" fn call_mut(&mut self, a: (A,B)) -> Value {
    match self {
      Function2::S(func) => {
        let av = a.0.into_value();
        let bv = a.1.into_value();
        func.call_mut((av,bv))
      }
      _ => unreachable!()
    }
  }
}

impl<A,B> FnMut<(&A,&B)> for Function2
  where
    A: for<'a> IntoValueRef<'a>,
    B: for<'b> IntoValueRef<'b>,
{
  default extern "rust-call" fn call_mut(&mut self, a: (&A,&B)) -> Value {
    match self {
      Function2::R(func) => {
        let av = a.0.into_value_ref();
        let bv = a.1.into_value_ref();
        func.call_mut((av,bv))
      }
      _ => unreachable!()
    }
  }
}

impl<'a,'b> FnMut<(R<'a>,R<'b>)> for Function2 {
  default extern "rust-call" fn call_mut(&mut self, (a,b): (R<'a>,R<'b>)) -> Value {
    match self {
      Function2::R(func) => {
        func.call_mut((a,b))
      }
      _ => unreachable!()
    }
  }
}

impl<'a,A,B> FnMut<(&'a A, &'a B)> for Function2R
  where
    A: IntoValueRef<'a>,
    B: IntoValueRef<'a>,
{
  default extern "rust-call" fn call_mut(&mut self, a: (&'a A, &'a B)) -> ValueRef<'a> {
    let av = a.0.into_value_ref();
    let bv = a.1.into_value_ref();
    self.inner.call_mut((av,bv))
  }
}

// Fn


impl<A,B> Fn<(A,B)> for Function2
  where
    A: IntoValue,
    B: IntoValue,
{
  default extern "rust-call" fn call(&self, a: (A,B)) -> Value {
    match self {
      Function2::S(func) => {
        let av = a.0.into_value();
        let bv = a.1.into_value();
        func.call((av,bv))
      }
      _ => unreachable!()
    }
  }
}

impl<A,B> Fn<(&A,&B)> for Function2
  where
    A: for<'a> IntoValueRef<'a>,
    B: for<'b> IntoValueRef<'b>,
{
  default extern "rust-call" fn call(&self, a: (&A,&B)) -> Value {
    match self {
      Function2::R(func) => {
        let av = a.0.into_value_ref();
        let bv = a.1.into_value_ref();
        func.call((av,bv))
      }
      _ => unreachable!()
    }
  }
}

impl<'a,'b> Fn<(R<'a>,R<'b>)> for Function2 {
  default extern "rust-call" fn call(&self, (a,b): (R<'a>, R<'b>)) -> Value {
    match self {
      Function2::R(func) => {
        func.call((a,b))
      }
      _ => unreachable!()
    }
  }
}

impl<'a,A,B> Fn<(&'a A, &'a B)> for Function2R
  where
    A: IntoValueRef<'a>,
    B: IntoValueRef<'a>,
{
  default extern "rust-call" fn call(&self, a: (&'a A, &'a B)) -> ValueRef<'a> {
    let av = a.0.into_value_ref();
    let bv = a.1.into_value_ref();
    self.inner.call((av,bv))
  }
}


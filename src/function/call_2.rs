use crate::value::*;
use crate::value_ref::*;
use crate::function::*;
use crate::function::helper_macros::*;

type R<'a> = ValueRef<'a>;

// FnOnce

impl<A,B> FnOnce<(A,B)> for Function
  where
    A: 'static,
    B: 'static,
{
  type Output = Value;

  default extern "rust-call" fn call_once(self, a: (A,B)) -> Value {
    f2!(self).call_once_s(a)
  }
}

impl<A,B> FnOnce<(&A,&B)> for Function
  where
    A: 'static,
    B: 'static,
{
  extern "rust-call" fn call_once(self, a: (&A,&B)) -> Value {
    f2!(self).call_once_r(a)
  }
}

impl FnOnce<(R<'_>,R<'_>)> for Function {
  extern "rust-call" fn call_once(self, a: (R,R)) -> Value {
    f2!(self).call_once_rr(a)
  }
}


// FnMut

impl<A,B> FnMut<(A,B)> for Function
  where
    A: 'static,
    B: 'static,
{
  default extern "rust-call" fn call_mut(&mut self, a: (A,B)) -> Value {
    f2!(self).call_mut_s(a)
  }
}

impl<A,B> FnMut<(&A,&B)> for Function
  where
    A: 'static,
    B: 'static,
{
  extern "rust-call" fn call_mut(&mut self, a: (&A,&B)) -> Value {
    f2!(self).call_mut_r(a)
  }
}

impl FnMut<(R<'_>,R<'_>)> for Function {
  extern "rust-call" fn call_mut(&mut self, a: (R,R)) -> Value {
    f2!(self).call_mut_rr(a)
  }
}


// Fn

impl<A,B> Fn<(A,B)> for Function
  where
    A: 'static,
    B: 'static,
{
  default extern "rust-call" fn call(&self, a: (A,B)) -> Value {
    f2!(&self).call_s(a)
  }
}

impl<A,B> Fn<(&A,&B)> for Function
  where
    A: 'static,
    B: 'static,
{
  extern "rust-call" fn call(&self, a: (&A,&B)) -> Value {
    f2!(&self).call_r(a)
  }
}

impl Fn<(R<'_>,R<'_>)> for Function {
  extern "rust-call" fn call(&self, a: (R,R)) -> Value {
    f2!(&self).call_rr(a)
  }
}


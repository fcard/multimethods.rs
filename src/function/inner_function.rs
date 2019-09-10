use crate::value::*;
use crate::value_ref::*;

// Constructors

pub trait InnerFunctionStaticNew<Args,F> {
  fn new_s(func: F) -> Self;
}

pub trait InnerFunctionRefNew<Args,F> {
  fn new_r(func: F) -> Self;
}

pub trait InnerFunctionRefReturnNew<Args,F>: Sized {
  fn new(func: F) -> Self;
}


// Calls

pub trait InnerFunctionStaticCalls<Args> {
  type Args = Args;

  fn call_once_s(self, a: Self::Args) -> Value;

  fn call_mut_s(&mut self, a: Self::Args) -> Value;

  fn call_s(&self, a: Self::Args) -> Value;
}


pub trait InnerFunctionRefCalls<Args> {
  fn call_once_r(self, a: Args) -> Value;

  fn call_mut_r(&mut self, a: Args) -> Value;

  fn call_r(&self, a: Args) -> Value;
}


pub trait InnerFunctionValueRefCalls<Args> {
  fn call_once_rr(self, a: Args) -> Value;

  fn call_mut_rr(&mut self, a: Args) -> Value;

  fn call_rr(&self, a: Args) -> Value;
}


pub trait InnerFunctionRefReturnCalls<'r, Args>: Sized {
  type Args = Args;

  fn call_once(self, a: Self::Args) -> ValueRef<'r>;

  fn call_mut(&mut self, a: Self::Args) -> ValueRef<'r>;

  fn call(&self, a: Self::Args) -> ValueRef<'r>;
}

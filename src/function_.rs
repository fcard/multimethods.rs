use crate::value::*;
use crate::value_ref::*;
use crate::function_0::*;
use crate::function_1::*;
use crate::function_2::*;

type R<'a> = ValueRef<'a>;

use std::ops::BitAnd;

pub enum Function {
  F0(Function0),
  F1(Function1),
  F1R(Function1R),
  F2(Function2),
  F2R(Function2R),
}


pub trait RefFn<'a, Args> {
  fn r(&self, a: Args) -> ValueRef<'a>;
}

pub macro new_function {
  // Zero arguments

  //-- Owned Arguments

  (|| $body: expr) => {
    Function::F0(Function0::new(||$body))
  },

  (|| -> $R: ty  { $body: expr }) => {
    Function::F0(Function0::new(|| -> $R { $body }))
  },

  //-- Reference Arguments

  (| | $body: expr) => {
    Function::F0(Function0::new(||$body))
  },

  (| | -> $R: ty  { $body: expr }) => {
    Function::F0(Function0::new(|| -> $R { $body }))
  },


  // One Argument

  //-- Owned Arguments

  (|$a: ident$(: $A: ty)?| $body: expr) => {
    Function::F1(
      Function1::s(
        |$a$(: $A)*| $body
      )
    )
  },

  (|$a: ident$(: $A: ty)?| -> $R: ty { $body: expr }) => {
    Function::F1(
      Function1::s(
        |$a$(: $A)*| -> $R { $body }
      )
    )
  },

  //-- Reference Arguments

  (&|$a: ident$(: $A: ty)?| $body: expr) => {
    Function::F1(
      Function1::r(
        |$a$(: &$A)*| $body
      )
    )
  },

  (&|$a: ident$(: $A: ty)?| -> $R: ty { $body: expr }) => {
    Function::F1(
      Function1::r(
        |$a$(: &$A)*| -> $R { $body }
      )
    )
  },

  //-- Reference Arguments and Return value

  (&&|$a: ident$(: $A: ty)?| -> $R: ty { $body: expr }) => {
    Function::F1R(
      Function1R::new(
        |$a$(: &$A)*| -> &$R { $body }
      )
    )
  },


  // Two Arguments

  //-- Owned Arguments

  (|$a: ident$(: $A: ty)?, $b: ident$(: $B: ty)?| $body: expr) => {
    Function::F2(
      Function2::s(
        |$a$(: $A)*,$b$(: $B)*| $body
      )
    )
  },

  (|$a: ident$(: $A: ty)?, $b: ident$(: $B: ty)?| -> $R: ty { $body: expr }) => {
    Function::F2(
      Function2::s(
        |$a$(: $A)*,$b$(: $B)*| -> $R { $body }
      )
    )
  },

  //-- Reference Arguments

  (&|$a: ident$(: $A: ty)?, $b: ident$(: $B: ty)?| $body: expr) => {
    Function::F2(
      Function2::r(
        |$a$(: &$A)*,$b$(: &$B)*| $body
      )
    )
  },

  (&|$a: ident$(: $A: ty)?, $b: ident$(: $B: ty)?| -> $R: ty { $body: expr }) => {
    Function::F2(
      Function2::r(
        |$a$(: &$A)*,$b$(: &$B)*| -> $R { $body }
      )
    )
  },

  //-- Reference Arguments and Return value

  (&&|$a: ident$(: $A: ty)?, $b: ident$(: $B: ty)?| -> $R: ty { $body: expr }) => {
    Function::F2R(
      Function2R::new(
        |$a$(: &$A)*,$b$(: &$B)*| -> &$R { $body }
      )
    )
  }
}



impl FnOnce<()> for Function {
  type Output = Value;

  extern "rust-call" fn call_once(self, _: ()) -> Value {
    match self {
      Function::F0(func) => {
        func()
      }
      _ => unreachable!()
    }
  }
}

impl<A> FnOnce<(A,)> for Function
  where
    A: 'static,
{
  type Output = Value;

  default extern "rust-call" fn call_once(self, (a,): (A,)) -> Value {
    match self {
      Function::F1(func) => {
        func(a)
      }
      _ => unreachable!()
    }
  }
}

impl<A> FnOnce<(&A,)> for Function
  where
    A: 'static,
{
  extern "rust-call" fn call_once(self, (a,): (&A,)) -> Value {
    match self {
      Function::F1(func) => {
        func(a)
      }
      _ => unreachable!()
    }
  }
}

impl FnOnce<(R<'_>,)> for Function {
  extern "rust-call" fn call_once(self, (a,): (R,)) -> Value {
    match self {
      Function::F1(func) => {
        func(a)
      }
      _ => unreachable!()
    }
  }
}

impl<A,B> FnOnce<(A,B)> for Function
where
  A: 'static,
  B: 'static
{
  type Output = Value;

  default extern "rust-call" fn call_once(self, (a,b): (A,B)) -> Value {
    match self {
      Function::F2(func) => {
        func(a,b)
      }
      _ => unreachable!()
    }
  }
}


impl<A,B> FnOnce<(&A, &B)> for Function
where
  A: 'static,
  B: 'static
{
  extern "rust-call" fn call_once(self, (a,b): (&A,&B)) -> Value {
    match self {
      Function::F2(func) => {
        func(a,b)
      }
      _ => panic!("p")
    }
  }
}

impl<'a> FnOnce<(R<'a>, R<'a>)> for Function {
  extern "rust-call" fn call_once(self, (a,b): (R,R)) -> Value {
    match self {
      Function::F2(func) => {
        func(a,b)
      }
      _ => panic!("p")
    }
  }
}

// FnMut

impl FnMut<()> for Function {
  extern "rust-call" fn call_mut(&mut self, _: ()) -> Value {
    match self {
      Function::F0(func) => {
        func()
      }
      _ => unreachable!()
    }
  }
}

impl<A> FnMut<(A,)> for Function
  where
    A: 'static,
{
  default extern "rust-call" fn call_mut(&mut self, (a,): (A,)) -> Value {
    match self {
      Function::F1(func) => {
        func(a)
      }
      _ => unreachable!()
    }
  }
}

impl<A> FnMut<(&A,)> for Function
  where
    A: 'static,
{
  extern "rust-call" fn call_mut(&mut self, (a,): (&A,)) -> Value {
    match self {
      Function::F1(func) => {
        func(a)
      }
      _ => unreachable!()
    }
  }
}

impl FnMut<(R<'_>,)> for Function {
  extern "rust-call" fn call_mut(&mut self, (a,): (R,)) -> Value {
    match self {
      Function::F1(func) => {
        func(a)
      }
      _ => unreachable!()
    }
  }
}

impl<A,B> FnMut<(A,B)> for Function
where
  A: 'static,
  B: 'static
{
  default extern "rust-call" fn call_mut(&mut self, (a,b): (A,B)) -> Value {
    match self {
      Function::F2(func) => {
        func(a,b)
      }
      _ => unreachable!()
    }
  }
}


impl<A,B> FnMut<(&A, &B)> for Function
where
  A: 'static,
  B: 'static
{
  extern "rust-call" fn call_mut(&mut self, (a,b): (&A,&B)) -> Value {
    match self {
      Function::F2(func) => {
        func(a,b)
      }
      _ => panic!("p")
    }
  }
}

impl<'a> FnMut<(R<'a>, R<'a>)> for Function {
  extern "rust-call" fn call_mut(&mut self, (a,b): (R,R)) -> Value {
    match self {
      Function::F2(func) => {
        func(a,b)
      }
      _ => panic!("p")
    }
  }
}

// Fn

impl Fn<()> for Function {
  extern "rust-call" fn call(&self, _: ()) -> Value {
    match self {
      Function::F0(func) => {
        func()
      }
      _ => unreachable!()
    }
  }
}

impl<A> Fn<(A,)> for Function
  where
    A: 'static,
{
  default extern "rust-call" fn call(&self, (a,): (A,)) -> Value {
    match self {
      Function::F1(func) => {
        func(a)
      }
      _ => unreachable!()
    }
  }
}

impl<A> Fn<(&A,)> for Function
  where
    A: 'static,
{
  extern "rust-call" fn call(&self, (a,): (&A,)) -> Value {
    match self {
      Function::F1(func) => {
        func(a)
      }
      _ => unreachable!()
    }
  }
}

impl<'a> Fn<(R<'a>,)> for Function {
  extern "rust-call" fn call(&self, (a,): (R,)) -> Value {
    match self {
      Function::F1(func) => {
        func(a)
      }
      _ => unreachable!()
    }
  }
}

impl<A,B> Fn<(A,B)> for Function
where
  A: 'static,
  B: 'static
{
  default extern "rust-call" fn call(&self, (a,b): (A,B)) -> Value {
    match self {
      Function::F2(func) => {
        func(a,b)
      }
      _ => unreachable!()
    }
  }
}


impl<A,B> Fn<(&A, &B)> for Function
where
  A: 'static,
  B: 'static
{
  extern "rust-call" fn call(&self, (a,b): (&A,&B)) -> Value {
    match self {
      Function::F2(func) => {
        func(a,b)
      }
      _ => panic!("p")
    }
  }
}

impl<'a> Fn<(R<'a>, R<'a>)> for Function {
  extern "rust-call" fn call(&self, (a,b): (R,R)) -> Value {
    match self {
      Function::F2(func) => {
        func(a,b)
      }
      _ => panic!("p")
    }
  }
}

impl<'a,A> BitAnd<(&'a A,)> for &Function
  where
    A: 'static,
{
  type Output = ValueRef<'a>;

  fn bitand(self, (a,): (&'a A,)) -> ValueRef<'a> {
    match self {
      Function::F1R(func) => {
        func(a)
      }
      _ => unreachable!()
    }
  }
}

impl<'a,A,B> BitAnd<(&'a A, &'a B)> for &Function
  where
    A: 'static,
    B: 'static,
{
  type Output = ValueRef<'a>;

  fn bitand(self, (a,b): (&'a A, &'a B)) -> ValueRef<'a> {
    match self {
      Function::F2R(func) => {
        func(a,b)
      }
      _ => unreachable!()
    }
  }
}

/*
impl<'a,'b,A,B> RefFn<'b, (&'a A, &'b B)> for Function
  where
    A: 'static,
    B: 'static,
{
  fn r(&self, (a,b): (&'a A, &'b B)) -> ValueRef<'b> {
    match self {
      Function::F2R1(func) => {
        func(a,b)
      }
      _ => unreachable!()
    }
  }
}
*/

use crate::value::*;
use crate::function::*;

// FnOnce

impl FnOnce<()> for Function {
  type Output = Value;

  extern "rust-call" fn call_once(self, _: ()) -> Value {
    match self {
      Function::F0(func) => {
        func.call_once(())
      }
      _ => unreachable!()
    }
  }
}


// FnMut

impl FnMut<()> for Function {
  extern "rust-call" fn call_mut(&mut self, _: ()) -> Value {
    match self {
      Function::F0(func) => {
        func.call_mut(())
      }
      _ => unreachable!()
    }
  }
}

// Fn

impl Fn<()> for Function {
  extern "rust-call" fn call(&self, _: ()) -> Value {
    match self {
      Function::F0(func) => {
        func.call(())
      }
      _ => unreachable!()
    }
  }
}


use crate::value::*;
use crate::value_ref::*;
use crate::function::*;

// General Helpers

macro as_value_ref_a($L: lifetime, $a: tt) { ValueRef<$L> }

macro as_value_ref($a: tt) { ValueRef<'_> }

macro as_value($a: tt) { Value }

// Destructuring the Function Enum

pub macro get_variant($t: ident, $func: expr, $variant: ident) {
  match $func {
    $t::$variant(f) => f,
    _ => panic!("Not a {}::{}", stringify!($t), stringify!($variant))
  }
}

pub macro get_func_variant($func: expr, $variant: ident) {
  get_variant!(Function, $func, $variant)
}

pub macro f0($func: expr) {
  get_func_variant!($func, F0)
}

pub macro f1($func: expr) {
  get_func_variant!($func, F1)
}

pub macro f1r($func: expr) {
  get_func_variant!($func, F1R)
}

pub macro f2($func: expr) {
  get_func_variant!($func, F2)
}

pub macro f2r($func: expr) {
  get_func_variant!($func, F2R)
}


pub macro k() {}

// Defining methods

// -- Constructors

pub macro static_constructor($Func: ident, $name: ident;  $($arg: ident: $T: ident),*) {
  impl<$($T,)* R,F> InnerFunctionStaticNew<($($T,)*),R,F> for $Func
    where
      $($T: FromValue,)*
      R: IntoValue,
      F: Fn($($T),*) -> R + 'static
  {
    fn new_s(func: F) -> $Func {
      $Func::S(
        box move |$($arg),*| {
          func($($T::from_value($arg)),*).into_value()
        }
      )
    }
  }
}

pub macro ref_constructor($Func: ident, $name: ident;  $($arg: ident: $T: ident),*) {
  impl<$($T,)* R,F> InnerFunctionRefNew<($($T,)*),R,F> for $Func
    where
      $($T: for<'a> FromValueRef<'a>,)*
      R: IntoValue,
      F: Fn($(&$T),*) -> R + 'static
  {
    fn new_r(func: F) -> $Func {
      $Func::R(
        box move |$($arg),*| {
          func($($T::from_value_ref($arg)),*).into_value()
        }
      )
    }
  }
}

// -- Calls

pub macro static_calls(
  $Func: ident,
  $variant: ident,
  $once: ident,
  $mut: ident,
  $fn: ident;

  $($a: ident: $T: ident),*) {

  impl<$($T),*> InnerFunctionStaticCalls<($($T,)*)> for $Func
    where
      $($T: IntoValue,)*
  {
    type Args = ($($T,)*);

    // -- call_once

    fn call_once_s(self, ($($a,)*): Self::Args) -> Value {
      get_variant!($Func, self, $variant).call_once(($($a.into_value(),)*))
    }

    // -- call_mut

    fn call_mut_s(&mut self, ($($a,)*): Self::Args) -> Value {
      get_variant!($Func, self, $variant).call_mut(($($a.into_value(),)*))
    }

    // -- call

    fn call_s(&self, ($($a,)*): Self::Args) -> Value {
      get_variant!($Func, self, $variant).call(($($a.into_value(),)*))
    }
  }
}


pub macro ref_calls(
  $Func: ident,
  $variant: ident,
  $once: ident,
  $mut: ident,
  $fn: ident;

  $($a: ident: $T: ident),*) {

  impl<$($T),*> InnerFunctionRefCalls<($(&$T,)*)> for $Func
    where
      $($T: for<'a> IntoValueRef<'a>,)*
  {
    // -- call_once

    fn call_once_r(self, ($($a,)*): ($(&$T,)*)) -> Value {
      get_variant!($Func, self, $variant).call_once(($($a.into_value_ref(),)*))
    }

    // -- call_mut

    fn call_mut_r(&mut self, ($($a,)*): ($(&$T,)*)) -> Value {
      get_variant!($Func, self, $variant).call_mut(($($a.into_value_ref(),)*))
    }

    // -- call

    fn call_r(&self, ($($a,)*): ($(&$T,)*)) -> Value {
      get_variant!($Func, self, $variant).call(($($a.into_value_ref(),)*))
    }
  }
}

pub macro value_ref_calls(
  $Func: ident,
  $variant: ident,
  $once: ident,
  $mut: ident,
  $fn: ident;

  $($a: ident),*) {

  impl InnerFunctionValueRefCalls<($(as_value_ref!($a),)*)> for $Func {
    // -- call_once

    fn call_once_rr(self, ($($a,)*): ($(as_value_ref!($a),)*)) -> Value {
      get_variant!($Func, self, $variant).call_once(($($a,)*))
    }

    // -- call_mut

    fn call_mut_rr(&mut self, ($($a,)*): ($(as_value_ref!($a),)*)) -> Value {
      get_variant!($Func, self, $variant).call_mut(($($a,)*))
    }

    // -- call

    fn call_rr(&self, ($($a,)*): ($(as_value_ref!($a),)*)) -> Value {
      get_variant!($Func, self, $variant).call(($($a,)*))
    }
  }
}


pub macro impl_function($Func: ident($($a: ident: $T: ident),*)) {
  static_constructor!($Func, new_s; $($a: $T),*);
  ref_constructor!   ($Func, new_r; $($a: $T),*);
  static_calls!      ($Func, S, call_once_s,  call_mut_s,  call_s;  $($a: $T),*);
  ref_calls!         ($Func, R, call_once_r,  call_mut_r,  call_r;  $($a: $T),*);
  value_ref_calls!   ($Func, R, call_once_rr, call_mut_rr, call_rr; $($a),*);
}


// Functions that return references

pub macro impl_ref_function($Func: ident($($a: ident: $T: ident),*)) {
  // Constructor

  impl<$($T,)* R,F> InnerFunctionRefReturnNew<($($T,)*),R,F> for $Func
    where
      $($T: for<'a> FromValueRef<'a> + 'static,)*
      R: for<'a> IntoValueRef<'a> + 'static,
      F: for<'a> Fn($(&'a $T),*) -> &'a R + 'static
  {
    fn new(func: F) -> Self {
      $Func {
        inner: box move |$($a),*| {
          func($($T::from_value_ref($a)),*).into_value_ref()
        }
      }
    }
  }

  // Calls

  impl<'r, $($T),*> InnerFunctionRefReturnCalls<'r, ($(&'r $T,)*)> for $Func
    where
      $($T: IntoValueRef<'r>,)*
  {
    type Args = ($(&'r $T,)*);

    // -- call_once

    fn call_once(self, ($($a,)*): Self::Args) -> ValueRef<'r> {
      self.inner.call_once(($($a.into_value_ref(),)*))
    }

    // -- call_mut

    fn call_mut(&mut self, ($($a,)*): Self::Args) -> ValueRef<'r> {
      self.inner.call_mut(($($a.into_value_ref(),)*))
    }

    // -- call

    fn call(&self, ($($a,)*): Self::Args) -> ValueRef<'r> {
      self.inner.call(($($a.into_value_ref(),)*))
    }
  }
}

pub macro fnbox_static($($a: ident),*) {
  Box<dyn Fn($(as_value!($a)),*) -> Value>
}

pub macro fnbox_ref($($a: ident),*) {
  Box<dyn Fn($(as_value_ref!($a)),*) -> Value>
}

pub macro fnbox_ref_return($($a: ident),*) {
  Box<dyn for<'a> Fn($(as_value_ref_a!('a, $a)),*) -> ValueRef<'a>>
}

// Implement Fn traits

pub macro impl_fn_traits(variant=$variant:ident; $($T: ident),*) {

  // FnOnce

  // -- Static arguments

  impl<$($T),*> FnOnce<($($T,)*)> for Function
    where
      $($T: 'static,)*
  {
    type Output = Value;

    default extern "rust-call" fn call_once(self, a: ($($T,)*)) -> Value {
      get_func_variant!(self, $variant).call_once_s(a)
    }
  }

  // -- Reference arguments

  impl<$($T),*> FnOnce<($(&$T,)*)> for Function
    where
      $($T: 'static,)*
  {
    extern "rust-call" fn call_once(self, a: ($(&$T,)*)) -> Value {
      get_func_variant!(self, $variant).call_once_r(a)
    }
  }

  // -- ValueRef arguments

  impl FnOnce<($(as_value_ref!($T),)*)> for Function {
    extern "rust-call" fn call_once(self, a: ($(as_value_ref!($T),)*)) -> Value {
      get_func_variant!(self, $variant).call_once_rr(a)
    }
  }

  // FnMut

  // -- Static arguments

  impl<$($T),*> FnMut<($($T,)*)> for Function
    where
      $($T: 'static,)*
  {
    default extern "rust-call" fn call_mut(&mut self, a: ($($T,)*)) -> Value {
      get_func_variant!(self, $variant).call_mut_s(a)
    }
  }

  // -- Reference arguments

  impl<$($T),*> FnMut<($(&$T,)*)> for Function
    where
      $($T: 'static,)*
  {
    extern "rust-call" fn call_mut(&mut self, a: ($(&$T,)*)) -> Value {
      get_func_variant!(self, $variant).call_mut_r(a)
    }
  }

  // -- ValueRef arguments

  impl FnMut<($(as_value_ref!($T),)*)> for Function {
    extern "rust-call" fn call_mut(&mut self, a: ($(as_value_ref!($T),)*)) -> Value {
      get_func_variant!(self, $variant).call_mut_rr(a)
    }
  }

  // Fn

  // -- Static arguments

  impl<$($T),*> Fn<($($T,)*)> for Function
    where
      $($T: 'static,)*
  {
    default extern "rust-call" fn call(&self, a: ($($T,)*)) -> Value {
      get_func_variant!(self, $variant).call_s(a)
    }
  }

  // -- Reference arguments

  impl<$($T),*> Fn<($(&$T,)*)> for Function
    where
      $($T: 'static,)*
  {
    extern "rust-call" fn call(&self, a: ($(&$T,)*)) -> Value {
      get_func_variant!(self, $variant).call_r(a)
    }
  }

  // -- ValueRef arguments

  impl Fn<($(as_value_ref!($T),)*)> for Function {
    extern "rust-call" fn call(&self, a: ($(as_value_ref!($T),)*)) -> Value {
      get_func_variant!(self, $variant).call_rr(a)
    }
  }
}


pub macro impl_ref_fn($variant: ident, $($T: ident),*) {
  impl<'a,$($T),*> RefFn<'a, ($(&'a $T,)*)> for Function
    where
      $($T: 'static),*
  {
    fn r(&self, a: ($(&'a $T,)*)) -> ValueRef<'a> {
      match self {
        Function::$variant(func) => {
          func.call(a)
        }
        _ => panic!("Not a Function::{}", stringify!($variant))
      }
    }
  }
}


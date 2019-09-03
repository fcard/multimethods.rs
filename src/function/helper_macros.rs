use crate::value::*;
use crate::value_ref::*;
use crate::function::*;

// General Helpers

macro ignore_first($a:tt, $b:tt) {$b}

macro as_value_ref($a: tt) { ValueRef<'_> }

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
  pub fn $name<$($T),*,R,F>(func: F) -> $Func
    where
      $($T: FromValue,)*
      R: IntoValue,
      F: Fn($($T),*) -> R + 'static
  {
    $Func::S(
      box move |$($arg),*| {
        func($($T::from_value($arg)),*).into_value()
      }
    )
  }
}

pub macro ref_constructor($Func: ident, $name: ident;  $($arg: ident: &$L: lifetime $T: ident),*) {
  pub fn $name<$($T),*,R,F>(func: F) -> $Func
    where
      $($T: for<$L> FromValueRef<$L>,)*
      R: IntoValue,
      F: for<$($L),*> Fn($(&$L $T),*) -> R + 'static
  {
    $Func::R(
      box move |$($arg),*| {
        func($($T::from_value_ref($arg)),*).into_value()
      }
    )
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

  // -- call_once

  pub fn $once<$($T),*>(self, ($($a,)*): ($($T,)*)) -> Value
    where
      $($T: IntoValue,)*
  {
    get_variant!($Func, self, $variant).call_once(($($a.into_value(),)*))
  }

  // -- call_mut

  pub fn $mut<$($T),*>(&mut self, ($($a,)*): ($($T,)*)) -> Value
    where
      $($T: IntoValue,)*
  {
    get_variant!($Func, self, $variant).call_mut(($($a.into_value(),)*))
  }

  // -- call

  pub fn $fn<$($T),*>(&self, ($($a,)*): ($($T,)*)) -> Value
    where
      $($T: IntoValue,)*
  {
    get_variant!($Func, self, $variant).call(($($a.into_value(),)*))
  }
}


pub macro ref_calls(
  $Func: ident,
  $variant: ident,
  $once: ident,
  $mut: ident,
  $fn: ident;

  $($a: ident: $T: ident),*) {

  // -- call_once

  pub fn $once<$($T),*>(self, ($($a,)*): ($(&$T,)*)) -> Value
    where
      $($T: for<'a> IntoValueRef<'a>,)*
  {
    get_variant!($Func, self, $variant).call_once(($($a.into_value_ref(),)*))
  }

  // -- call_mut

  pub fn $mut<$($T),*>(&mut self, ($($a,)*): ($(&$T,)*)) -> Value
    where
      $($T: for<'a> IntoValueRef<'a>,)*
  {
    get_variant!($Func, self, $variant).call_mut(($($a.into_value_ref(),)*))
  }

  // -- call

  pub fn $fn<$($T),*>(&self, ($($a,)*): ($(&$T,)*)) -> Value
    where
      $($T: for<'a> IntoValueRef<'a>,)*
  {
    get_variant!($Func, self, $variant).call(($($a.into_value_ref(),)*))
  }
}

pub macro value_ref_calls(
  $Func: ident,
  $variant: ident,
  $once: ident,
  $mut: ident,
  $fn: ident;

  $($a: ident),*) {

  // -- call_once

  pub fn $once(self, ($($a,)*): ($(ignore_first!($a,ValueRef),)*)) -> Value {
    get_variant!($Func, self, $variant).call_once(($($a,)*))
  }

  // -- call_mut

  pub fn $mut(&mut self, ($($a,)*): ($(ignore_first!($a, ValueRef),)*)) -> Value {
    get_variant!($Func, self, $variant).call_mut(($($a,)*))
  }

  // -- call

  pub fn $fn(&self, ($($a,)*): ($(ignore_first!($a, ValueRef),)*)) -> Value {
    get_variant!($Func, self, $variant).call(($($a,)*))
  }
}

pub macro impl_function {

  // Function that only accepts static arguments
  (
    type = $Func: ident;
    parameters   = [ $($a: ident: $T: ident),* ];
    constructors = [ $new_s: ident ];
    static_calls = [
      variant = $S: ident;
      $call_once_s: ident, $call_mut_s: ident, $call_s: ident
    ];
  ) => {
    impl $Func {
      static_constructor!($Func, $new_s; $($a: $T),*);
      static_calls!      ($Func, $S, $call_once_s, $call_mut_s, $call_s; $($a: $T),*);
    }
  },

  // Static and references function
  (
    type = $Func: ident;
    constructors = [ $new_s: ident, $new_r: ident];
    parameters   = [ $($a: ident: $(&$L: lifetime)? $T: ident),* ];
    static_calls = [
      variant = $S: ident;
      $call_once_s: ident, $call_mut_s: ident, $call_s: ident
    ];
    ref_calls = [
      variant = $R: ident;
      $call_once_r:  ident, $call_mut_r:  ident, $call_r:  ident,
      $call_once_rr: ident, $call_mut_rr: ident, $call_rr: ident$(,)?
    ];
  ) => {
    impl $Func {
      static_constructor!($Func, $new_s; $($a: $T),*);
      ref_constructor!   ($Func, $new_r; $($a: $(&$L )*$T),*);
      static_calls!      ($Func, $S, $call_once_s, $call_mut_s, $call_s; $($a: $T),*);
      ref_calls!         ($Func, $R, $call_once_r,  $call_mut_r,  $call_r;  $($a: $T),*);
      value_ref_calls!   ($Func, $R, $call_once_rr, $call_mut_rr, $call_rr; $($a),*);
    }
  }
}


// Functions that return references

pub macro impl_ref_function(
  type = $Func: ident;
  constructor = $new: ident;
  parameters  = [ $($a: ident: $T: ident),* ];
  calls       = [ $call_once: ident, $call_mut: ident, $call: ident ]
) {
  impl $Func {

    // Constructor

    pub fn $new<$($T,)* R, F>(func: F) -> Self
      where
        $($T: for<'a> FromValueRef<'a> + 'static,)*
        R: for<'a> IntoValueRef<'a> + 'static,
        F: for<'a> Fn($(&'a $T),*) -> &'a R + 'static
    {
      $Func {
        inner: box move |$($a),*| {
          func($($T::from_value_ref($a)),*).into_value_ref()
        }
      }
    }

    // Calls

    // -- call_once

    pub fn $call_once<'a, $($T),*>(self, ($($a,)*): ($(&'a $T,)*)) -> ValueRef<'a>
      where
        $($T: IntoValueRef<'a>,)*
    {
      self.inner.call_once(($($a.into_value_ref(),)*))
    }

    // -- call_mut

    pub fn $call_mut<'a, $($T),*>(&mut self, ($($a,)*): ($(&'a $T,)*)) -> ValueRef<'a>
      where
        $($T: IntoValueRef<'a>,)*
    {
      self.inner.call_mut(($($a.into_value_ref(),)*))
    }

    // -- call

    pub fn $call<'a, $($T),*>(&self, ($($a,)*): ($(&'a $T,)*)) -> ValueRef<'a>
      where
        $($T: IntoValueRef<'a>,)*
    {
      self.inner.call(($($a.into_value_ref(),)*))
    }
  }
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


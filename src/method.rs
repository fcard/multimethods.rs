use std::any::*;
use std::ops::*;
use std::collections::HashMap;
use std::sync::{Mutex};
use std::cell::{UnsafeCell};
use crate::value::*;
use crate::value_ref::*;
use crate::types::*;
use crate::function::*;
use lazy_static::*;

type TypeKey = (((TypeId, TypeId, TypeId, TypeId), bool), bool);

pub type Method = Function;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenericFunction{
  pub id: usize,
  pub rr: RefGenericFunction
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefGenericFunction(usize);

impl RefGenericFunction {
  fn id(self) -> GenericFunction {
    GenericFunction {
      id: self.0,
      rr: self
    }
  }
}

pub struct FunctionTable {
  id: Mutex<usize>,
  lock: Mutex<()>,
  functions: UnsafeCell<HashMap<GenericFunction, MethodTable>>,
}

pub struct MethodTable {
  methods: HashMap<TypeKey, Method>
}

unsafe impl Sync for MethodTable {}
unsafe impl Sync for FunctionTable {}

impl MethodTable {
  pub fn new() -> Self {
    MethodTable {
      methods: HashMap::new()
    }
  }

  pub fn insert(&mut self, key: TypeKey, value: Method) {
    self.methods.insert(key, value);
  }

  pub fn get(&self, key: TypeKey) -> &Method {
    unwrap_method(self.methods.get(&key))
  }

  pub fn get_mut(&mut self, key: TypeKey) -> &mut Method {
    unwrap_method(self.methods.get_mut(&key))
  }

  pub fn remove(&mut self, key: TypeKey) -> Method {
    unwrap_method(self.methods.remove(&key))
  }
}

fn unwrap_method<M>(m: Option<M>) -> M {
  match m {
    Some(m) => m,
    None => panic!("Method not found")
  }
}

impl FunctionTable {
  fn new() -> Self {
    FunctionTable {
      id: Mutex::new(0),
      lock: Mutex::new(()),

      functions: UnsafeCell::new(
        HashMap::new()
      ),
    }
  }

  pub fn with_functions_mut<R,F>(&self, f: F) -> R
    where
      F: FnOnce(&mut HashMap<GenericFunction, MethodTable>) -> R
  {
    let _lock = self.lock.lock();
    f(unsafe {&mut *self.functions.get()})
  }

  pub fn functions(&self) -> &HashMap<GenericFunction, MethodTable> {
    unsafe {&*self.functions.get()}
  }

  pub fn remove(&self, fun: GenericFunction, types: TypeKey) -> Method {
    self.with_functions_mut(
      |functions| {
        functions.get_mut(&fun).unwrap().remove(types)
      }
    )
  }

  pub fn with_mut<R,F>(&self, fun: GenericFunction, types: TypeKey, func: F) -> R
    where
      F: FnOnce(&mut Method) -> R
  {
    self.with_functions_mut(
      |functions| {
        func(functions.get_mut(&fun).unwrap().get_mut(types))
      }
    )
  }

  pub fn get(&self, fun: GenericFunction, types: TypeKey) -> &Method {
    self.functions().get(&fun).unwrap().get(types)
  }

  pub fn new_function(&self, table: MethodTable) -> GenericFunction {
    let mut id = self.id.lock().unwrap();
    let fun = GenericFunction{ id:*id, rr: RefGenericFunction(*id) };

    self.with_functions_mut(
      move |functions| {
        functions.insert(fun, table);
      }
    );
    *id += 1;
    fun
  }
}

impl<Args> FnOnce<Args> for GenericFunction
  where
    Args: Types,
    Function: FnOnce<Args, Output=Value>
{
  type Output = Value;

  extern "rust-call" fn call_once(self, a: Args) -> Value {
    GENERIC_FUNCTIONS.remove(self, (a.types(), false)).call_once(a)
  }
}

impl<Args> FnMut<Args> for GenericFunction
  where
    Args: Types,
    Function: FnMut<Args, Output=Value>
{
  extern "rust-call" fn call_mut(&mut self, a: Args) -> Value {
    GENERIC_FUNCTIONS.with_mut(*self, (a.types(), false), |method| method.call_mut(a))
  }
}

impl<Args> Fn<Args> for GenericFunction
  where
    Args: Types,
    Function: Fn<Args, Output=Value>
{
  extern "rust-call" fn call(&self, a: Args) -> Value {
    GENERIC_FUNCTIONS.get(*self, (a.types(), false)).call(a)
  }
}

impl<'a,A> FnOnce<(&'a A,)> for RefGenericFunction
  where
    A: 'static
{
  type Output = ValueRef<'a>;

  extern "rust-call" fn call_once(self, a: (&'a A,)) -> ValueRef<'a> {
    GENERIC_FUNCTIONS.remove(self.id(), (a.types(), true)).r(a)
  }
}

impl<'a,A> FnMut<(&'a A,)> for RefGenericFunction
  where
    A: 'static,
{
  extern "rust-call" fn call_mut(&mut self, a: (&'a A,)) -> ValueRef<'a> {
    GENERIC_FUNCTIONS.with_mut(self.id(), (a.types(), true), |method| method.r(a))
  }
}

impl<'a,A> Fn<(&'a A,)> for RefGenericFunction
  where
    A: 'static
{
  extern "rust-call" fn call(&self, a: (&'a A,)) -> ValueRef<'a> {
    GENERIC_FUNCTIONS.get(self.id(), (a.types(), true)).r(a)
  }
}


impl<'a,A,B> FnOnce<(&'a A, &'a B)> for RefGenericFunction
  where
    A: 'static,
    B: 'static,
{
  type Output = ValueRef<'a>;

  extern "rust-call" fn call_once(self, a: (&'a A, &'a B)) -> ValueRef<'a> {
    GENERIC_FUNCTIONS.remove(self.id(), (a.types(), true)).r(a)
  }
}

impl<'a,A,B> FnMut<(&'a A, &'a B)> for RefGenericFunction
  where
    A: 'static,
    B: 'static,
{
  extern "rust-call" fn call_mut(&mut self, a: (&'a A, &'a B)) -> ValueRef<'a> {
    GENERIC_FUNCTIONS.with_mut(self.id(), (a.types(), true), |method| method.r(a))
  }
}

impl<'a,A,B> Fn<(&'a A, &'a B)> for RefGenericFunction
  where
    A: 'static,
    B: 'static,
{
  extern "rust-call" fn call(&self, a: (&'a A, &'a B)) -> ValueRef<'a> {
    GENERIC_FUNCTIONS.get(self.id(), (a.types(), true)).r(a)
  }
}


pub macro tid($T: ty) {
  TypeId::of::<$T>()
}

pub macro type_key {
  ()                               => { (tid!( !), tid!( !), tid!( !), tid!( !)) },
  ($A: ty)                         => { (tid!($A), tid!( !), tid!( !), tid!( !)) },
  ($A: ty, $B: ty)                 => { (tid!($A), tid!($B), tid!( !), tid!( !)) },
  ($A: ty, $B: ty, $C: ty)         => { (tid!($A), tid!($B), tid!($C), tid!( !)) },
  ($A: ty, $B: ty, $C: ty, $D: ty) => { (tid!($A), tid!($B), tid!($C), tid!($D)) }
}

pub macro new_generic_function {
  (name=$name: ident;
   $(
     $($mref: ident)+($($arg: ident: $T: ty),*)$(-> $R: ty)? $block: block
    )*
  ) => {
    lazy_static! {
      pub static ref $name: GenericFunction = {
        let mut table = MethodTable::new();
        $(
          let method: Method =
            method_def!($($mref)*($($arg: $T),*)$(-> $R)* { $block });

          let type_key =
            ((type_key!($($T),*), is_ref!($($mref)*)), is_return_ref!($($mref)*));

          table.insert(type_key, method);
        )*
        GENERIC_FUNCTIONS.new_function(table)
      };
    }
  }
}

pub macro method_def {
  (method($($arg: ident: $T: ty),*) $(-> $R: ty)? { $block: expr }) => {
    new_function!(
      |$($arg: $T),*| $( -> $R)* { $block }
    )
  },

  (ref method($($arg: ident: $T: ty),*) $(-> $R: ty)? { $block: expr }) => {
    new_function!(
      &|$($arg: $T),*| $( -> $R)* { $block }
    )
  },

  (ref return method($($arg: ident: $T: ty),*) -> $R: ty { $block: expr }) => {
    new_function!(
      &&|$($arg: $T),*| -> $R { $block }
    )
  },
}

pub macro is_ref {
  (method)     => {false},
  (ref method) => {true},
  (ref return method) => {true}
}

pub macro is_return_ref {
  (method)     => {false},
  (ref method) => {false},
  (ref return method) => {true}
}

lazy_static! {
  pub static ref GENERIC_FUNCTIONS: FunctionTable = FunctionTable::new();
}

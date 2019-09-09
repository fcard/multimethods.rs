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

type ConcreteTypeKey = ((TypeTuple, bool), bool);
type AbstractTypeKey = (TypeMatchTuple, bool);

type TypeKeys = (ConcreteTypeKey, AbstractTypeKey);

pub trait AsTypeKey {
  fn as_concrete_type_key(&self) -> ConcreteTypeKey;
  fn as_abstract_type_key(&self) -> AbstractTypeKey;

  fn type_keys(&self) -> TypeKeys {
    (self.as_concrete_type_key(),
     self.as_abstract_type_key())
  }
}

impl<T> AsTypeKey for (&T, bool)
  where
    T: Types + AsTypeMatches
{
  fn as_concrete_type_key(&self) -> ConcreteTypeKey {
    (self.0.types(), self.1)
  }

  fn as_abstract_type_key(&self) -> AbstractTypeKey {
    (self.0.as_type_matches(), self.1)
  }
}

impl AsTypeKey for TypeKeys {
  fn as_concrete_type_key(&self) -> ConcreteTypeKey {
    self.0.clone()
  }

  fn as_abstract_type_key(&self) -> AbstractTypeKey {
    self.1.clone()
  }
}


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
  methods: HashMap<ConcreteTypeKey, Method>,
  abstracts: TypeMatchTree<Method>
}

unsafe impl Sync for MethodTable {}
unsafe impl Sync for FunctionTable {}


macro get_method($self: expr, $key: expr, $get: ident) {
  match $self.methods.$get(&$key.as_concrete_type_key()) {
    Some(method) => method,
    None => {
      let abs = $key.as_abstract_type_key();
      unwrap_method($self.abstracts.$get(&abs.0, abs.1))
    }
  }
}


impl MethodTable {
  pub fn new() -> Self {
    MethodTable {
      methods: HashMap::new(),
      abstracts: TypeMatchTree::new()
    }
  }

  pub fn insert(&mut self, key: ConcreteTypeKey, value: Method) {
    self.methods.insert(key, value);
  }

  pub fn insert_abstract(&mut self, key: AbstractTypeKey, value: Method) {
    self.abstracts.insert(key.0, value, key.1);
  }

  pub fn get<T: AsTypeKey>(&self, key: T) -> &Method {
    get_method!(self, key, get)
  }

  pub fn get_mut<T: AsTypeKey>(&mut self, key: T) -> &mut Method {
    get_method!(self, key, get_mut)
  }

  pub fn remove<T: AsTypeKey>(&mut self, key: T) -> Method {
    get_method!(self, key, remove)
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

  pub fn remove<T: AsTypeKey>(&self, fun: GenericFunction, key: T) -> Method {
    self.with_functions_mut(
      |functions| {
        functions.get_mut(&fun).unwrap().remove(key)
      }
    )
  }

  pub fn with_mut<R,F>(&self, fun: GenericFunction, key: TypeKeys, func: F) -> R
    where
      F: FnOnce(&mut Method) -> R
  {
    self.with_functions_mut(
      |functions| {
        func(functions.get_mut(&fun).unwrap().get_mut(key))
      }
    )
  }

  pub fn get<T: AsTypeKey>(&self, fun: GenericFunction, key: T) -> &Method {
    self.functions().get(&fun).unwrap().get(key)
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
    Args: Types + AsTypeMatches,
    Function: FnOnce<Args, Output=Value>
{
  type Output = Value;

  extern "rust-call" fn call_once(self, a: Args) -> Value {
    GENERIC_FUNCTIONS.remove(self, (&a, false)).call_once(a)
  }
}

impl<Args> FnMut<Args> for GenericFunction
  where
    Args: Types + AsTypeMatches,
    Function: FnMut<Args, Output=Value>
{
  extern "rust-call" fn call_mut(&mut self, a: Args) -> Value {
    GENERIC_FUNCTIONS.with_mut(*self, (&a, false).type_keys(), |method| method.call_mut(a))
  }
}

impl<Args> Fn<Args> for GenericFunction
  where
    Args: Types + AsTypeMatches,
    Function: Fn<Args, Output=Value>
{
  extern "rust-call" fn call(&self, a: Args) -> Value {
    GENERIC_FUNCTIONS.get(*self, (&a, false)).call(a)
  }
}

impl<'a,A> FnOnce<(&'a A,)> for RefGenericFunction
  where
    A: 'static
{
  type Output = ValueRef<'a>;

  extern "rust-call" fn call_once(self, a: (&'a A,)) -> ValueRef<'a> {
    GENERIC_FUNCTIONS.remove(self.id(), (&a, true)).r(a)
  }
}

impl<'a,A> FnMut<(&'a A,)> for RefGenericFunction
  where
    A: 'static,
{
  extern "rust-call" fn call_mut(&mut self, a: (&'a A,)) -> ValueRef<'a> {
    GENERIC_FUNCTIONS.with_mut(self.id(), (&a, true).type_keys(), |method| method.r(a))
  }
}

impl<'a,A> Fn<(&'a A,)> for RefGenericFunction
  where
    A: 'static
{
  extern "rust-call" fn call(&self, a: (&'a A,)) -> ValueRef<'a> {
    GENERIC_FUNCTIONS.get(self.id(), (&a, true)).r(a)
  }
}


impl<'a,A,B> FnOnce<(&'a A, &'a B)> for RefGenericFunction
  where
    A: 'static,
    B: 'static,
{
  type Output = ValueRef<'a>;

  extern "rust-call" fn call_once(self, a: (&'a A, &'a B)) -> ValueRef<'a> {
    GENERIC_FUNCTIONS.remove(self.id(), (&a, true)).r(a)
  }
}

impl<'a,A,B> FnMut<(&'a A, &'a B)> for RefGenericFunction
  where
    A: 'static,
    B: 'static,
{
  extern "rust-call" fn call_mut(&mut self, a: (&'a A, &'a B)) -> ValueRef<'a> {
    GENERIC_FUNCTIONS.with_mut(self.id(), (&a, true).type_keys(), |method| method.r(a))
  }
}

impl<'a,A,B> Fn<(&'a A, &'a B)> for RefGenericFunction
  where
    A: 'static,
    B: 'static,
{
  extern "rust-call" fn call(&self, a: (&'a A, &'a B)) -> ValueRef<'a> {
    GENERIC_FUNCTIONS.get(self.id(), (&a, true)).r(a)
  }
}


macro t($T: ty) {
  TypeId::of::<$T>()
}

macro ts($($T: ty),*) {
  ($(t!($T),)*)
}

pub macro type_key {
  () => {
    ts!( !, !, !, !, !, !, !, !, !, !, !, !)
  },

  ($A:ty) => {
    ts!($A, !, !, !, !, !, !, !, !, !, !, !)
  },

  ($A:ty,$B:ty) => {
    ts!($A,$B, !, !, !, !, !, !, !, !, !, !)
  },

  ($A:ty,$B:ty,$C:ty) => {
    ts!($A,$B,$C, !, !, !, !, !, !, !, !, !)
  },

  ($A:ty,$B:ty,$C:ty,$D:ty) => {
    ts!($A,$B,$C,$D, !, !, !, !, !, !, !, !)
  },

  ($A:ty,$B:ty,$C:ty,$D:ty,$E:ty) => {
    ts!($A,$B,$C,$D,$E, !, !, !, !, !, !, !)
  },

  ($A:ty,$B:ty,$C:ty,$D:ty,$E:ty,$F:ty) => {
    ts!($A,$B,$C,$D,$E,$F, !, !, !, !, !, !)
  },

  ($A:ty,$B:ty,$C:ty,$D:ty,$E:ty,$F:ty,$G:ty) => {
    ts!($A,$B,$C,$D,$E,$F,$G, !, !, !, !, !)
  },

  ($A:ty,$B:ty,$C:ty,$D:ty,$E:ty,$F:ty,$G:ty,$H:ty) => {
    ts!($A,$B,$C,$D,$E,$F,$G,$H, !, !, !, !)
  },

  ($A:ty,$B:ty,$C:ty,$D:ty,$E:ty,$F:ty,$G:ty,$H:ty,$I:ty) => {
    ts!($A,$B,$C,$D,$E,$F,$G,$H,$I, !, !, !)
  },

  ($A:ty,$B:ty,$C:ty,$D:ty,$E:ty,$F:ty,$G:ty,$H:ty,$I:ty,$J:ty) => {
    ts!($A,$B,$C,$D,$E,$F,$G,$H,$I,$J, !, !)
  },

  ($A:ty,$B:ty,$C:ty,$D:ty,$E:ty,$F:ty,$G:ty,$H:ty,$I:ty,$J:ty,$K:ty) => {
    ts!($A,$B,$C,$D,$E,$F,$G,$H,$I,$J,$K, !)
  },

  ($A:ty,$B:ty,$C:ty,$D:ty,$E:ty,$F:ty,$G:ty,$H:ty,$I:ty,$J:ty,$K:ty,$L:ty) => {
    ts!($A,$B,$C,$D,$E,$F,$G,$H,$I,$J,$K,$L)
  },
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
  (method($($arg: tt: $T: ty),*) $(-> $R: ty)? { $block: expr }) => {
    new_function!(
      |$($arg: $T),*| $( -> $R)* { $block }
    )
  },

  (ref method($($arg: tt: $T: ty),*) $(-> $R: ty)? { $block: expr }) => {
    new_function!(
      &|$($arg: $T),*| $( -> $R)* { $block }
    )
  },

  (ref return method($($arg: tt: $T: ty),*) -> $R: ty { $block: expr }) => {
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

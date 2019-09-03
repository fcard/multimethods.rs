use std::any::*;
use crate::value::*;
use crate::value_ref::*;

pub trait TypeOf {
  fn is_ref(&self)  -> bool;
  fn type_of(&self) -> TypeId;
}

impl<T: 'static> TypeOf for T {
  default fn is_ref(&self) -> bool {
    false
  }

  default fn type_of(&self) -> TypeId {
    self.type_id()
  }
}


impl<T: 'static> TypeOf for &T {
  default fn is_ref(&self) -> bool {
    true
  }

  default fn type_of(&self) -> TypeId {
    (*self).type_id()
  }
}


impl<T: 'static> TypeOf for &&T {
  default fn is_ref(&self) -> bool {
    true
  }

  default fn type_of(&self) -> TypeId {
    (**self).type_id()
  }
}


impl<T: 'static> TypeOf for &&&T {
  default fn is_ref(&self) -> bool {
    true
  }

  default fn type_of(&self) -> TypeId {
    (***self).type_id()
  }
}


impl TypeOf for Value {
  default fn is_ref(&self) -> bool {
    false
  }

  default fn type_of(&self) -> TypeId {
    (&*self.inner).type_id()
  }
}


impl<'a> TypeOf for ValueRef<'a> {
  default fn is_ref(&self) -> bool {
    true
  }

  default fn type_of(&self) -> TypeId {
    (*self.inner).type_id()
  }
}


pub trait Types {
  fn types(&self) -> ((TypeId, TypeId, TypeId, TypeId), bool);
}


impl Types for () {
  fn types(&self) -> ((TypeId, TypeId, TypeId, TypeId), bool) {
    ((TypeId::of::<!>(),
      TypeId::of::<!>(),
      TypeId::of::<!>(),
      TypeId::of::<!>()),
     false)
  }
}

impl<A> Types for (A,)
  where
    A: TypeOf,
{
  fn types(&self) -> ((TypeId, TypeId, TypeId, TypeId), bool) {
    ((self.0.type_of(),
      TypeId::of::<!>(),
      TypeId::of::<!>(),
      TypeId::of::<!>()),

     (self.0.is_ref()))
  }
}

impl<A,B> Types for (A,B)
  where
    A: TypeOf,
    B: TypeOf,
{
  fn types(&self) -> ((TypeId, TypeId, TypeId, TypeId), bool) {
    ((self.0.type_of(),
      self.1.type_of(),
      TypeId::of::<!>(),
      TypeId::of::<!>()),

     (self.0.is_ref() ||
      self.1.is_ref()))
  }
}

impl<A,B,C> Types for (A,B,C)
  where
    A: TypeOf,
    B: TypeOf,
    C: TypeOf,
{
  fn types(&self) -> ((TypeId, TypeId, TypeId, TypeId), bool) {
    ((self.0.type_of(),
      self.1.type_of(),
      self.2.type_of(),
      TypeId::of::<!>()),

     (self.0.is_ref() ||
      self.1.is_ref() ||
      self.2.is_ref()))
  }
}

impl<A,B,C,D> Types for (A,B,C,D)
  where
    A: TypeOf,
    B: TypeOf,
    C: TypeOf,
    D: TypeOf,
{
  fn types(&self) -> ((TypeId, TypeId, TypeId, TypeId), bool) {
    ((self.0.type_of(),
      self.1.type_of(),
      self.2.type_of(),
      self.3.type_of()),

     (self.0.is_ref() ||
      self.1.is_ref() ||
      self.2.is_ref() ||
      self.3.is_ref()))
  }
}


use std::any::*;
use crate::value::*;
use crate::value_ref::*;
use multimethods_helper_proc::*;

pub mod abs;
pub use abs::*;

pub mod type_match_tree;
pub use type_match_tree::*;

pub mod abstract_impl;
pub use abstract_impl::*;

pub mod type_value;
pub use type_value::*;


pub trait TypeOf {
  fn is_ref(&self)  -> bool;
  fn type_of(&self) -> TypeId;
  fn associated_is_ref() -> bool;
  fn associated_type_of() -> TypeId;
}

impl<T: 'static> TypeOf for T {
  default fn is_ref(&self) -> bool {
    T::associated_is_ref()
  }

  default fn type_of(&self) -> TypeId {
    T::associated_type_of()
  }

  default fn associated_is_ref() -> bool {
    false
  }

  default fn associated_type_of() -> TypeId {
    TypeId::of::<T>()
  }
}


impl<T: 'static> TypeOf for &T {
  default fn associated_is_ref() -> bool {
    true
  }

  default fn associated_type_of() -> TypeId {
    TypeId::of::<T>()
  }
}


impl<T: 'static> TypeOf for &&T {
  default fn associated_type_of() -> TypeId {
    TypeId::of::<T>()
  }
}


impl<T: 'static> TypeOf for &&&T {
  default fn associated_type_of() -> TypeId {
    TypeId::of::<T>()
  }
}


impl TypeOf for Value {
  default fn is_ref(&self) -> bool {
    false
  }

  default fn type_of(&self) -> TypeId {
    (&*self.inner).type_id()
  }

  default fn associated_type_of() -> TypeId {
    panic!("`Value` has no associated type id")
  }
}


impl<'a> TypeOf for ValueRef<'a> {
  default fn is_ref(&self) -> bool {
    true
  }

  default fn type_of(&self) -> TypeId {
    (*self.inner).type_id()
  }

  default fn associated_type_of() -> TypeId {
    panic!("`ValueRef` has no associated type id")
  }
}

pub type TypeTuple =
  (TypeId, TypeId, TypeId, TypeId,
   TypeId, TypeId, TypeId, TypeId,
   TypeId, TypeId, TypeId, TypeId);

pub trait Types {
  fn type_tuple(&self) -> TypeTuple;
  fn has_ref(&self) -> bool;

  fn types(&self) -> (TypeTuple, bool) {
    (self.type_tuple(), self.has_ref())
  }

  fn type0(&self)  -> ConcreteType;
  fn type1(&self)  -> ConcreteType;
  fn type2(&self)  -> ConcreteType;
  fn type3(&self)  -> ConcreteType;
  fn type4(&self)  -> ConcreteType;
  fn type5(&self)  -> ConcreteType;
  fn type6(&self)  -> ConcreteType;
  fn type7(&self)  -> ConcreteType;
  fn type8(&self)  -> ConcreteType;
  fn type9(&self)  -> ConcreteType;
  fn type10(&self) -> ConcreteType;
  fn type11(&self) -> ConcreteType;
}

impl_types!(0);
impl_types!(1);
impl_types!(2);
impl_types!(3);
impl_types!(4);
impl_types!(5);
impl_types!(6);
impl_types!(7);
impl_types!(8);
impl_types!(9);
impl_types!(10);
impl_types!(11);
impl_types!(12);


use std::any::*;
use crate::types::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeMatch {
  Concrete(ConcreteType),
  Abstract(AbstractType)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConcreteType {
  pub id: TypeId,
  pub is_ref: bool,
  pub parent: AbstractType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AbstractType {
  pub name: &'static str,
  pub parent: Option<&'static AbstractType>
}


pub const ANY: AbstractType =
  AbstractType { name: "ANY", parent: None };


pub macro new_abstract_type {
  ($($v: vis $name: ident$(: $supertype: expr)?),*$(,)?) => {
    $(
      $v const $name: AbstractType =
        AbstractType {
          name: stringify!($name),
          parent: Some(&parent_type!($($supertype)*))
        };
     )*
  },
}

macro parent_type {
  () => { ANY },
  ($supertype: expr) => { $supertype },
}


pub macro impl_abstract_type($($type: ty: $abstract: expr),*$(,)?) {
  $(
    impl SubType for $type {
      const TYPE: AbstractType = $abstract;
    }
  )*
}


pub macro Abstract($t: ident) {
  Value
}

pub trait SubType: TypeOf {
  const TYPE: AbstractType;

  fn concrete_type(&self) -> ConcreteType {
    ConcreteType {
      id:     self.type_of(),
      is_ref: self.is_ref(),
      parent: self.abstract_type()
    }
  }

  fn abstract_type(&self) -> AbstractType {
    Self::TYPE
  }

  fn associated_concrete_type() -> ConcreteType {
    ConcreteType {
      id:     Self::associated_type_of(),
      is_ref: Self::associated_is_ref(),
      parent: Self::associated_abstract_type()
    }
  }

  fn associated_abstract_type() -> AbstractType {
    Self::TYPE
  }
}

impl<T: TypeOf> SubType for T {
  default const TYPE: AbstractType = ANY;
}

impl TypeMatch {
  pub fn matches<T: SubType>(&self, ty: T) -> bool {
    match &self {
      TypeMatch::Concrete(c) => {
        (ty.type_of() == c.id) &&
          (ty.is_ref() == c.is_ref)
      }

      TypeMatch::Abstract(a) => {
        ty.abstract_type().is_subtype(a)
      }
    }
  }

  pub fn is_super_match(&self, m: &TypeMatch) -> bool {
    self == m || (
      match self {
        TypeMatch::Abstract(a) => {
          match m {
            TypeMatch::Concrete(c) => {
              c.parent.is_subtype(a)
            }

            TypeMatch::Abstract(b) => {
              b.is_subtype(a)
            }
          }
        }
        _ => false
      }
    )
  }
}


impl AbstractType {
  pub fn is_subtype(&self, ty: &AbstractType) -> bool {
    ty == self ||
      match &self.parent {
        Some(p) => p.is_subtype(ty),
        None => false
      }
  }
}

type TM = TypeMatch;
#[derive(Debug,Clone,PartialEq)]
pub enum TypeMatches {
  T0(),
  T1(TM),
  T2(TM,TM),
  T3(TM,TM,TM),
  T4(TM,TM,TM,TM),
  T5(TM,TM,TM,TM,TM),
  T6(TM,TM,TM,TM,TM,TM),
  T7(TM,TM,TM,TM,TM,TM,TM),
  T8(TM,TM,TM,TM,TM,TM,TM,TM),
  T9(TM,TM,TM,TM,TM,TM,TM,TM,TM),
  T10(TM,TM,TM,TM,TM,TM,TM,TM,TM,TM),
  T11(TM,TM,TM,TM,TM,TM,TM,TM,TM,TM,TM),
  T12(TM,TM,TM,TM,TM,TM,TM,TM,TM,TM,TM,TM),
  V1(TM),
  V2(TM,TM),
  V3(TM,TM,TM),
  V4(TM,TM,TM,TM),
  V5(TM,TM,TM,TM,TM),
  V6(TM,TM,TM,TM,TM,TM),
  V7(TM,TM,TM,TM,TM,TM,TM),
  V8(TM,TM,TM,TM,TM,TM,TM,TM),
  V9(TM,TM,TM,TM,TM,TM,TM,TM,TM),
  V10(TM,TM,TM,TM,TM,TM,TM,TM,TM,TM),
  V11(TM,TM,TM,TM,TM,TM,TM,TM,TM,TM,TM),
  V12(TM,TM,TM,TM,TM,TM,TM,TM,TM,TM,TM,TM),
}

pub type TypeMatchTuple =
  (TypeMatch, TypeMatch, TypeMatch, TypeMatch,
   TypeMatch, TypeMatch, TypeMatch, TypeMatch,
   TypeMatch, TypeMatch, TypeMatch, TypeMatch);

pub trait AsTypeMatches {
  fn as_type_matches(&self) -> TypeMatches;
}

impl AsTypeMatches for TypeMatches {
  fn as_type_matches(&self) -> TypeMatches {
    self.clone()
  }
}

impl<T: Types> AsTypeMatches for T {
  default fn as_type_matches(&self) -> TypeMatches {
    macro n {
      (0)  => {self.type0()},
      (1)  => {self.type1()},
      (2)  => {self.type2()},
      (3)  => {self.type3()},
      (4)  => {self.type4()},
      (5)  => {self.type5()},
      (6)  => {self.type6()},
      (7)  => {self.type7()},
      (8)  => {self.type8()},
      (9)  => {self.type9()},
      (10) => {self.type10()},
      (11) => {self.type11()},
    }

    macro concretes($variant: ident$(, $($n: tt),*)?) {
      TypeMatches::$variant($($(TypeMatch::Concrete(n!($n))),*)*)
    }

    match self.type_tuple() {
      TypeIds::T0(..)  => concretes!(T0),
      TypeIds::T1(..)  => concretes!(T1,  0),
      TypeIds::T2(..)  => concretes!(T2,  0, 1),
      TypeIds::T3(..)  => concretes!(T3,  0, 1, 2),
      TypeIds::T4(..)  => concretes!(T4,  0, 1, 2, 3),
      TypeIds::T5(..)  => concretes!(T5,  0, 1, 2, 3, 4),
      TypeIds::T6(..)  => concretes!(T6,  0, 1, 2, 3, 4, 5),
      TypeIds::T7(..)  => concretes!(T7,  0, 1, 2, 3, 4, 5, 6),
      TypeIds::T8(..)  => concretes!(T8,  0, 1, 2, 3, 4, 5, 6, 7),
      TypeIds::T9(..)  => concretes!(T9,  0, 1, 2, 3, 4, 5, 6, 7, 8),
      TypeIds::T10(..) => concretes!(T10, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9),
      TypeIds::T11(..) => concretes!(T11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10),
      TypeIds::T12(..) => concretes!(T12, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11),
    }
  }
}

pub fn matches_all<T: AsTypeMatches> (a: &TypeMatches, b: &T) -> bool {
  use TypeMatches::*;

  macro match_all($(($a: ident, $b: ident)),*) {
    true $(&& $a.is_super_match(&$b))*
  }

  match (a, &b.as_type_matches()) {
    (T0(),
     T0()) => true,

    (T1(a0),
     T1(b0)) =>
      match_all!((a0,b0)),

    (T2(a0, a1),
     T2(b0, b1)) =>
      match_all!((a0,b0), (a1,b1)),

    (T3(a0, a1, a2),
     T3(b0, b1, b2)) =>
      match_all!((a0,b0),(a1,b1),(a2,b2)),

    (T4(a0, a1, a2, a3),
     T4(b0, b1, b2, b3)) =>
      match_all!((a0,b0),(a1,b1),(a2,b2),(a3,b3)),

    (T5(a0, a1, a2, a3, a4),
     T5(b0, b1, b2, b3, b4)) =>
      match_all!((a0,b0),(a1,b1),(a2,b2),(a3,b3),(a4,b4)),

    (T6(a0, a1, a2, a3, a4, a5),
     T6(b0, b1, b2, b3, b4, b5)) =>
      match_all!((a0,b0),(a1,b1),(a2,b2),(a3,b3),(a4,b4),(a5,b5)),

    (T7(a0, a1, a2, a3, a4, a5, a6),
     T7(b0, b1, b2, b3, b4, b5, b6)) =>
      match_all!((a0,b0),(a1,b1),(a2,b2),(a3,b3),(a4,b4),(a5,b5),
                 (a6,b6)),

    (T8(a0, a1, a2, a3, a4, a5, a6, a7),
     T8(b0, b1, b2, b3, b4, b5, b6, b7)) =>
      match_all!((a0,b0),(a1,b1),(a2,b2),(a3,b3),(a4,b4),(a5,b5),
                 (a6,b6),(a7,b7)),

    (T9(a0, a1, a2, a3, a4, a5, a6, a7, a8),
     T9(b0, b1, b2, b3, b4, b5, b6, b7, b8)) =>
      match_all!((a0,b0),(a1,b1),(a2,b2),(a3,b3),(a4,b4),(a5,b5),
                 (a6,b6),(a7,b7),(a8,b8)),

    (T10(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9),
     T10(b0, b1, b2, b3, b4, b5, b6, b7, b8, b9)) =>
      match_all!((a0,b0),(a1,b1),(a2,b2),(a3,b3),(a4,b4),(a5,b5),
                 (a6,b6),(a7,b7),(a8,b8),(a9,b9)),

    (T11(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10),
     T11(b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10)) =>
      match_all!((a0,b0),(a1,b1),(a2,b2),(a3,b3),(a4, b4),(a5,b5),
                 (a6,b6),(a7,b7),(a8,b8),(a9,b9),(a10,b10)),

    (T12(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11),
     T12(b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11)) =>
      match_all!((a0,b0),(a1,b1),(a2,b2),(a3,b3),(a4, b4),  (a5, b5),
                 (a6,b6),(a7,b7),(a8,b8),(a9,b9),(a10,b10), (a11,b11)),


    (V1(av), b) => {
      match_vararg!(b, av)
    }

    (V2(a0, av), b) => {
      match_vararg!(b, a0, av)
    }

    (V3(a0, a1, av), b) => {
      match_vararg!(b, a0, a1, av)
    }

    (V4(a0, a1, a2, av), b) => {
      match_vararg!(b, a0, a1, a2, av)
    }

    (V5(a0, a1, a2, a3, av), b) => {
      match_vararg!(b, a0, a1, a2, a3, av)
    }

    (V6(a0, a1, a2, a3, a4, av), b) => {
      match_vararg!(b, a0, a1, a2, a3, a4, av)
    }

    (V7(a0, a1, a2, a3, a4, a5, av), b) => {
      match_vararg!(b, a0, a1, a2, a3, a4, a5, av)
    }

    (V8(a0, a1, a2, a3, a4, a5, a6, av), b) => {
      match_vararg!(b, a0, a1, a2, a3, a4, a5, a6, av)
    }

    (V9(a0, a1, a2, a3, a4, a5, a6, a7, av), b) => {
      match_vararg!(b, a0, a1, a2, a3, a4, a5, a6, a7, av)
    }

    (V10(a0, a1, a2, a3, a4, a5, a6, a7, a8, av), b) => {
      match_vararg!(b, a0, a1, a2, a3, a4, a5, a6, a7, a8, av)
    }

    (V11(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, av), b) => {
      match_vararg!(b, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, av)
    }

    (V12(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, av), b) => {
      match_vararg!(b, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, av)
    }

    _ => false
  }
}

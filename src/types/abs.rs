use std::any::*;
use crate::types::*;

#[derive(Clone, PartialEq)]
pub enum TypeMatch {
  Concrete(ConcreteType),
  Abstract(AbstractType)
}

#[derive(Clone, PartialEq)]
pub struct ConcreteType {
  pub id: TypeId,
  pub is_ref: bool,
  pub parent: AbstractType,
}

#[derive(Clone, PartialEq)]
pub struct AbstractType {
  pub parent: Option<&'static AbstractType>
}


pub struct Top;

pub const TOP: AbstractType =
  AbstractType { parent: None };


pub macro new_abstract_type {
  ($v: vis $name: ident) => {
    $v const $name: AbstractType =
      AbstractType { parent: Some(&TOP) };
  },

  ($v: vis $name: ident: $supertype: ident) => {
    $v const $name: AbstractType =
      AbstractType { parent: Some(&$supertype) }
  }
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
  default const TYPE: AbstractType = TOP;
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

pub type TypeMatchTuple =
  (TypeMatch, TypeMatch, TypeMatch, TypeMatch,
   TypeMatch, TypeMatch, TypeMatch, TypeMatch,
   TypeMatch, TypeMatch, TypeMatch, TypeMatch);

pub trait AsTypeMatches {
  fn as_type_matches(&self) -> TypeMatchTuple;
}

impl AsTypeMatches for TypeMatchTuple {
  fn as_type_matches(&self) -> TypeMatchTuple {
    self.clone()
  }
}

impl<T: Types> AsTypeMatches for T {
  default fn as_type_matches(&self) -> TypeMatchTuple {
    let concrete = |c| TypeMatch::Concrete(c);

    (concrete(self.type0()), concrete(self.type1()),  concrete(self.type2()),
     concrete(self.type3()), concrete(self.type4()),  concrete(self.type5()),
     concrete(self.type6()), concrete(self.type7()),  concrete(self.type8()),
     concrete(self.type9()), concrete(self.type10()), concrete(self.type11()))
  }
}

pub fn matches_all<T: AsTypeMatches> (a: &TypeMatchTuple, b: &T) -> bool {
  let (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11) = a;
  let (b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11) = b.as_type_matches();


  (a0.is_super_match(&b0) && a1.is_super_match(&b1)   && a2.is_super_match(&b2) &&
   a3.is_super_match(&b3) && a4.is_super_match(&b4)   && a5.is_super_match(&b5) &&
   a6.is_super_match(&b6) && a7.is_super_match(&b7)   && a8.is_super_match(&b8) &&
   a9.is_super_match(&b9) && a10.is_super_match(&b10) && a11.is_super_match(&b11))
}

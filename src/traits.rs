use crate::*;
use std::fmt::{Debug, Display, self};
use std::ops::*;
use std::marker::PhantomData;

// CLONE

macro impl_common_traits(
  clone=$clone: ident,
  debug=$debug: ident;
  $($T: ty),*) {

  #[__fmc]
  multifunction! {
    $(
      pub fn $clone(x: &$T) -> $T {
        x.clone()
      }
    )*
  }

  #[__fmc]
  multifunction! {
    $(
      pub fn $debug(x: $T) -> String {
        format!("{:?}", x)
      }

      pub fn $debug(x: &$T) -> String {
        format!("{:?}", x)
      }
    )*
  }
}

macro impl_display_traits(
  display=$display: ident;
  $($T: ty),*) {


  #[__fmc]
  multifunction! {
    $(
      pub fn $display(x: $T) -> String {
        format!("{}", x)
      }

      pub fn $display(x: &$T) -> String {
        format!("{}", x)
      }
    )*
  }
}

pub macro impl_operator_with_value($T: ty, $Trait: ident, $op: ident) {
  impl $Trait<Value> for $T {
    type Output = Value;

    fn $op(self, other: Value) -> Value {
      $op(self.into_value(), other)
    }
  }

  impl<'a> $Trait<ValueRef<'a>> for &'a $T {
    type Output = Value;

    fn $op(self, other: ValueRef<'a>) -> Value {
      $op(self.into_value_ref(), other)
    }
  }
}

pub macro impl_operator_generic_function($func: ident, $op: tt, $($T: ty),+) {
  #[__fmc]
  multifunction! {
    $(
      pub fn $func(x: $T, y: $T) -> $T {
        x $op y
      }

      pub fn $func(x: $T, y: $T) -> $T {
        x $op y
      }
    )*
  }
}

pub macro impl_operator_method($Trait: ident, $func: ident, $op: tt, $($T: ty),+) {
  impl_operator_generic_function!($func, $op, $($T),*);
  $(
    impl_operator_with_value!($T, $Trait, $func);
  )*
}


macro impl_math_traits(
  add=$add: ident;
  sub=$sub: ident;
  mul=$mul: ident;
  div=$div: ident;
  $($T: ty),*) {

  impl_operator_method!(Add, $add, +, $($T),*);
  impl_operator_method!(Sub, $sub, -, $($T),*);
  impl_operator_method!(Mul, $mul, *, $($T),*);
  impl_operator_method!(Div, $div, /, $($T),*);
}

macro impl_bit_traits(
  xor=$xor: ident;
  and=$and: ident;
   or=$or:  ident;
  $($T: ty),*) {

  impl_operator_method!(BitXor, $xor, ^, $($T),*);
  impl_operator_method!(BitAnd, $and, &, $($T),*);
  impl_operator_method!(BitOr,  $or,  |, $($T),*);
}

macro impl_partial_eq_trait(
  eq=$eq: ident;
  ne=$ne: ident;
  $($T: ty),*) {

  #[__fmc]
  multifunction! {
    $(
      pub fn $eq(x: &$T, y: &$T) -> bool {
        x == y
      }
    )*
  }

  #[__fmc]
  multifunction! {
    $(
      pub fn $ne(x: &$T, y: &$T) -> bool {
        x == y
      }
    )*
  }

  $(
    impl PartialEq<Value> for $T
      where
        Self: 'static
    {
      fn eq(&self, other: &Value) -> bool {
        bool::from_value(eq(self, other))
      }

      fn ne(&self, other: &Value) -> bool {
        bool::from_value(ne(self, other))
      }
    }

    impl<'a> PartialEq<ValueRef<'a>> for &'a $T
      where
        Self: 'static
    {
      fn eq(&self, other: &ValueRef<'a>) -> bool {
        bool::from_value(eq(self, other))
      }

      fn ne(&self, other: &ValueRef<'a>) -> bool {
        bool::from_value(ne(self, other))
      }
    }
  )*
}

impl_common_traits! {
  clone=clone,
  debug=debug;

  (), (Value,),
  (Value,Value,),
  (Value,Value,Value),
  (Value,Value,Value,Value),
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize,
  f32, f64, String, &'static str, bool,
  Vec<Value>
}


impl_display_traits! {
  display=display;

  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize,
  f32, f64, String, &'static str, bool
}

impl_math_traits! {
  add=add;
  sub=sub;
  mul=mul;
  div=div;

  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize,
  f32, f64
}

impl_bit_traits! {
  xor=bitxor;
  and=bitand;
  or=bitor;

  bool,
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize
}

impl_partial_eq_trait! {
  eq=eq;
  ne=ne;

  (), (Value,),
  (Value,Value,),
  (Value,Value,Value),
  (Value,Value,Value,Value),
  (Value,Value,Value,Value,Value),
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize,
  f32, f64, String, &'static str, bool,
  Vec<Value>
}

pub struct Type<T>(PhantomData<T>);

pub fn _type<T>() -> Type<T> {
  Type(PhantomData)
}

pub macro Type($T: ty) {
  _type::<$T>()
}

new_generic_function! {
  name=into;

  method(_t: Type<i64>, a: i32) -> i64 {
    i64::from(a)
  }

  method(_t: Type<i64>, a: i64) -> i64 {
    a
  }
}

impl Clone for Value {
  fn clone(&self) -> Value {
    clone(self)
  }
}

impl Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", String::from_value(debug(self.into_value_ref())))
  }
}

impl<'a> Debug for ValueRef<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", String::from_value(debug(*self)))
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", String::from_value(display(self.into_value_ref())))
  }
}

impl<'a> Display for ValueRef<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", String::from_value(display(*self)))
  }
}


pub macro impl_operator_for_value($Trait: ident, $op: ident $(,$ref: tt)?) {
  impl<A> $Trait<A> for Value
    where
      A: IntoValue
  {
    type Output = Value;

    fn $op($($ref)* self, other: $($ref)* A) -> Value {
      $op(self, other.into_value())
    }
  }

  impl<'a,A> $Trait<&'a A> for ValueRef<'a>
    where
      A: IntoValueRef<'a> + 'static
  {
    type Output = Value;

    fn $op($($ref)* self, other: $($ref)* &'a A) -> Value {
      $op(self, other.into_value_ref())
    }
  }
}


impl_operator_for_value!(Add, add);
impl_operator_for_value!(Sub, sub);
impl_operator_for_value!(Mul, mul);
impl_operator_for_value!(Div, div);
impl_operator_for_value!(BitXor, bitxor);
impl_operator_for_value!(BitAnd, bitand);
impl_operator_for_value!(BitOr, bitor);

impl<A> PartialEq<A> for Value
  where
    Self: 'static,
    A: 'static
{
  fn eq(&self, other: &A) -> bool {
    bool::from_value(eq(self.into_value_ref(), other.into_value_ref()))
  }
}

impl<'a, A> PartialEq<&'a A> for ValueRef<'a>
  where
    Self: 'static,
    A: 'static
{
  fn eq(&self, other: &&'a A) -> bool {
    bool::from_value(eq(self, other))
  }
}


impl<'a> PartialEq<ValueRef<'a>> for ValueRef<'a> {
  fn eq(&self, other: &ValueRef<'a>) -> bool {
    bool::from_value(eq(*self, *other))
  }
}


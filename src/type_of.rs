use crate::*;

macro impl_type_of($($T: ty),*$(,)?) {
  #[__fmc]
  multifunction! {
    $(
      pub fn type_of(_: $T) -> Type<$T> {
        Type![$T]
      }
    )*
  }
}

impl_type_of! {
  i8, i16, i32, i64, i128, isize,
  u8, u16, u32, u64, u128, usize,
  char, bool, Vec<Value>,
}


use crate::*;

macro impl_into(name=$name: ident; $($into: ty: [$($from: ty),*]),*$(,)?) {
  #[__fmc]
  multifunction! {
    $(
      $(
        pub fn $name(_: Type<$into>, a: $from) -> $into {
          <$into>::from(a)
        }
      )*
    )*
  }
}

impl_into! {
  name=into;

  i8:    [bool],
  i16:   [bool, i8, u8],
  i32:   [bool, i8, u8, i16, u16],
  i64:   [bool, i8, u8, i16, u16, i32, u32],
  i128:  [bool, i8, u8, i16, u16, i32, u32, i64, u64],
  isize: [bool, i8, u8, i16],

  u8:    [bool],
  u16:   [bool, u8],
  u32:   [bool, u8, u16, char],
  u64:   [bool, u8, u16, u32],
  u128:  [bool, u8, u16, u32, u64],
  usize: [bool, u8, u16],

  f32: [i8, u8, i16, u16],
  f64: [i8, u8, i16, u16, i32, u32, f32],

  char: [u8],

  Vec<u8>: [String],
}

use crate::types::*;

new_abstract_type! {
  pub NUMBER,
  pub REAL: NUMBER,
  pub FLOAT: REAL,
  pub INTEGER: REAL,
  pub SIGNED: INTEGER,
  pub UNSIGNED: INTEGER,

  pub ARRAY,
  pub VECTOR: ARRAY,
}

impl_abstract_type! {
  i8:   SIGNED,
  i16:  SIGNED,
  i32:  SIGNED,
  i64:  SIGNED,
  i128: SIGNED,
  u8:   UNSIGNED,
  u16:  UNSIGNED,
  u32:  UNSIGNED,
  u64:  UNSIGNED,
  u128: UNSIGNED,
  f32:  FLOAT,
  f64:  FLOAT,
}

impl<T: 'static> SubType for Vec<T> {
  const TYPE: AbstractType = VECTOR;
}


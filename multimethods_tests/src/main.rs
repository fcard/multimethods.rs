#![feature(never_type)]

use multimethods::*;

multifunction! {
  pub fn F(_: Abstract![ANY], _: Abstract![ANY]) -> &'static str {
    "any"
  }

  pub fn F(a: Abstract![INTEGER], b: Abstract![INTEGER]) -> Value {
    into(Type![i64], a + b)
  }
}

#[cfg(test)]
mod tests {
  use crate::*;

  #[test]
  fn test_f() {
    assert_eq!(F(1, 2), 3i64);
    assert_eq!(F(1i32, 2i32), 3i64);
    assert_eq!(F(1i16, 2i16), 3i64);
    assert_eq!(F("a",  2), "any");
    assert_eq!(type_of(F(1,2)), Type![i64]);
  }
}

fn main() {
  println!("{}", F(1,2));
  println!("{:?}", type_of(F(1,2)));
  println!("{:?}", F("a",2));
}


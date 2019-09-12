#![feature(specialization)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

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

#[cfg(test)]
mod readme {
  pub use multimethods::*;

  mod hello1 {
    use multimethods::{multifunction, FromValue};

    multifunction! {
      pub fn HELLO() -> String {
        "Hello, World!".to_string()
      }

      pub fn HELLO(x: &'static str) -> String {
        format!("Hello, {}!", x)
      }

      pub fn HELLO(x: String) -> String {
        format!("Hello, Stringified {}!", x)
      }
    }

    #[test]
    fn readme_hello1() {
      assert_eq!(String::from_value(HELLO()),                   "Hello, World!".to_string());
      assert_eq!(String::from_value(HELLO("John")),             "Hello, John!".to_string());
      assert_eq!(String::from_value(HELLO("John".to_string())), "Hello, Stringified John!".to_string());
    }
  }

  mod hi1 {
    use crate::readme::hello1::HELLO;
    use multimethods::{multifunction, multimethods, initialize_methods, FromValue};

    multifunction!(HI);

    multimethods! {
      key=NUMERIC;

      pub fn HI(x: i32) -> String {
        format!("Hi, Number {}!", x)
      }

      pub fn HELLO(x: i32) -> String {
        format!("Hello, Number {}!", x)
      }
    }

    #[test]
    fn readme_hi1() {
      initialize_methods(&NUMERIC);

      assert_eq!(String::from_value(HELLO(1)), "Hello, Number 1!".to_string());
      assert_eq!(String::from_value(HI(2)),    "Hi, Number 2!".to_string());
    }
  }

  mod ref1 {
    use multimethods::{multifunction, FromValue};

    multifunction! {
      fn F(x: i32, y: i32) -> i32 {
        x + y
      }

      fn F(x: &i32, y: &i32) -> i32 {
        x + y
      }
    }

    #[test]
    fn readme_ref1() {
      assert_eq!(i32::from_value(F(1,2)),   3);
      assert_eq!(i32::from_value(F(&1,&2)), 3);
    }
  }

  mod add_int1 {
    use multimethods::{multifunction, FromValue, Type, type_of};

    multifunction! {
      fn ADD_INT(x: i32, y: i32) -> i32 {
        x + y
      }

      fn ADD_INT(x: i64, y: i64) -> i64 {
        x + y
      }
    }

    #[test]
    fn readme_add_int1() {
      assert_eq!(i32::from_value(ADD_INT(1i32, 1i32)), 2i32);
      assert_eq!(i64::from_value(ADD_INT(1i64, 1i64)), 2i64);

      assert_eq!(type_of(ADD_INT(1i32, 1i32)), Type![i32]);
      assert_eq!(type_of(ADD_INT(1i64, 1i64)), Type![i64]);
    }

    #[test]
    #[should_panic]
    fn readme_add_int1_panic() {
      i32::from_value(ADD_INT(1i64, 1i64));
    }
  }

  mod traits {
    use multimethods::{multifunction, multimethods, initialize_methods, clone, debug, IntoValue, FromValue};

    #[derive(PartialEq, Clone, Copy, Debug)]
    struct MyType(i32);

    multifunction! {
      pub fn ADD_INT(x: i32, y: i32) -> i32 {
        x + y
      }
    }


    multimethods! {
      key=MY_TYPE_TRAITS;

      pub fn debug(x: &MyType) -> String {
        format!("{:?}", x)
      }

      pub fn clone(x: &MyType) -> MyType {
        x.clone()
      }
    }


    #[test]
    fn readme_traits() {
      assert_eq!(ADD_INT(1,2) + 3, 6);

      initialize_methods(&MY_TYPE_TRAITS);
      assert_eq!(MyType::from_value((&MyType(1).into_value()).clone()), MyType(1));
      assert_eq!(format!("{:?}", (&MyType(1).into_value()).clone()), "MyType(1)".to_string());
    }
  }

  mod ref_ret {
    use multimethods::*;

    multifunction! {
      pub fn SELF(x: String) -> String {
        x
      }

      pub fn SELF(x: &String) -> &String {
        x
      }
    }

    #[test]
    fn readme_ref_ret() {
      let a = "a".to_string();
      let b = "b".to_string();

      assert_eq!(SELF(a), "a".to_string());
      assert_eq!(String::from_value_ref((SELF.rr)(&b)), &b);
    }
  }

  mod abstract1 {
    use multimethods::{multifunction, Abstract, ANY, NUMBER};

    multifunction! {
      pub fn DESCRIBE(_x: Abstract![ANY]) -> String {
        "I am of any type!".to_string()
      }

      pub fn DESCRIBE(_x: Abstract![NUMBER]) -> String {
        "I am of some numeric type!".to_string()
      }

      pub fn DESCRIBE(x: i32) -> String {
        format!("I am a i32 of value {}!", x)
      }
    }

    #[test]
    fn readme_abstract1() {
      assert_eq!(DESCRIBE("a"),  "I am of any type!".to_string());
      assert_eq!(DESCRIBE(1.0),  "I am of some numeric type!".to_string());
      assert_eq!(DESCRIBE(1i32), "I am a i32 of value 1!".to_string());
    }
  }


  mod abstract2 {
    use multimethods::{Value, multifunction, new_abstract_type, impl_abstract_type, Abstract, ANY, IntoValue};

    struct MyType1(i32);
    struct MyType2(String);
    struct MyType3(Value);

    new_abstract_type! {
      MY_ABSTRACT1,
      MY_ABSTRACT2: MY_ABSTRACT1,
    }

    impl_abstract_type! {
      MyType1: MY_ABSTRACT1,
      MyType2: MY_ABSTRACT2,
      MyType3: MY_ABSTRACT2
    }

    multifunction! {
      pub fn DESCRIBE(_x: Abstract![ANY]) -> &'static str {
        "any"
      }

      pub fn DESCRIBE(_x: Abstract![MY_ABSTRACT1]) -> &'static str {
        "abstract 1"
      }

      pub fn DESCRIBE(_x: Abstract![MY_ABSTRACT2]) -> &'static str {
        "abstract 2"
      }

      pub fn DESCRIBE(_x: MyType3) -> &'static str {
        "my type 3"
      }
    }

    #[test]
    fn readme_abstract2() {
      assert_eq!(DESCRIBE("a"),                       "any");
      assert_eq!(DESCRIBE(MyType1(1)),                "abstract 1");
      assert_eq!(DESCRIBE(MyType2("a".to_string())),  "abstract 2");
      assert_eq!(DESCRIBE(MyType3("a".into_value())), "my type 3");
    }
  }

  mod abstract3 {
    use multimethods::{multifunction, AbstractType, new_abstract_type, SubType, Abstract, ANY};

    struct MyCollection<T>(Vec<T>);
    new_abstract_type!(MY_COLLECTION);

    impl<T: 'static> SubType for MyCollection<T> {
      const TYPE: AbstractType = MY_COLLECTION;
    }

    multifunction! {
      pub fn IS_MY_COLLECTION(_: Abstract![ANY]) -> bool {
        false
      }

      pub fn IS_MY_COLLECTION(_: Abstract![MY_COLLECTION]) -> bool {
        true
      }
    }

    #[test]
    fn readme_abstract3() {
      let coll1 = MyCollection(vec![1,2,3]);
      let coll2 = MyCollection(vec!["a","b"]);

      assert_eq!(IS_MY_COLLECTION(1),     false);
      assert_eq!(IS_MY_COLLECTION(coll1), true);
      assert_eq!(IS_MY_COLLECTION(coll2), true);
    }
  }

  mod vararg {
    use multimethods::{multifunction, FromValue};

    multifunction! {
      fn SUM(args: Vararg![i32]) -> i32 {
        args.iter().sum()
      }
    }

    multifunction! {
      fn PRINT_ALL(args: Vararg![]) -> Vec<String>  {
        let mut result = Vec::new();
        for arg in args {
          result.push(format!("{}", arg))
        }
        result
      }
    }

    #[test]
    fn readme_vararg() {
      assert_eq!(SUM(1, 2, 3), 6);
      assert_eq!(
        <Vec<String>>::from_value(PRINT_ALL("a", 2)),
        vec!["a".to_string(), "2".to_string()]
      );
    }
  }
}

fn main() {
  println!("{}", F(1,2));
  println!("{:?}", type_of(F(1,2)));
  println!("{:?}", F("a",2));
}


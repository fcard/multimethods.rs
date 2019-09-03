#![feature(fn_traits)]
#![feature(specialization)]
#![feature(unboxed_closures)]
#![feature(box_syntax)]
#![feature(trait_alias)]
#![feature(decl_macro)]
#![feature(associated_type_defaults)]
#![feature(never_type)]
#![allow(non_upper_case_globals)]

pub mod value;
pub use value::*;

pub mod value_ref;
pub use value_ref::*;

pub mod function;
pub use function::*;

pub mod types;
pub use types::*;

pub mod method;
pub use method::*;

pub mod traits;
pub use traits::*;

new_generic_function! {
  name=hello;

  method() -> String {
    "Hello, world!".to_string()
  }

  method(a: i32) -> i32 {
    a + 1
  }

  method(a: i32, b: i32) -> i32 {
    a + b
  }

  ref method(a: i32) -> i32 {
    a + 2
  }

  ref method(s: String) -> String {
    let mut s2 = s.clone();
    s2.push_str(" and then...");
    s2
  }

  ref return method(s: String) -> String {
    s
  }

  method(s: String) {
    println!("{}", s);
  }
}

new_generic_function! {
  name=debugin;

  method(a: i32) -> String {
    format!("{:?}", a)
  }

  ref method(a: i32) -> String {
    format!("{:?}", a)
  }
}

fn main() {
  let a0: i32 = 1;
  let a1 = "abc";
  let s = String::from("hello world!");

  let rr = &a0;
  let kk = rr.into_value_ref();
  let f0 = Function1::new_r(|_i: &i32| "hello");
  println!("{:?}", debug(kk));
  let k = new_function!(&&|a: String| -> String { a });

  println!("{:?}", hello(&s));
  println!("{:?}", (hello.rr)(&s));
  println!("{}", hello(&s) == hello(&s));
  println!("{}", eq(&s, &s));
  println!("{:?}", (1.into_value()) == (2.into_value()));
  println!("{}", eq(hello(&s).into_value_ref(), hello(&s).into_value_ref()));
  println!("{}", (hello.rr)(&s) == (hello.rr)(&s));



  /*
  println!("{:?}", String::from_value(debug((&a0).into_value_ref())));
  println!("{:?}", a0.into_value());
  println!("{:?}", (&a0).into_value_ref());
  println!("{:?}", (a1).into_value());
  println!("{:?}", i32::from_value(a0.into_value()));

  let f1s   = new_function!(|x:i32| x + 1);
  let f1r   = new_function!(&|x:i32| (*x) + 1);
  let f2s   = new_function!(|x:i32,y:i32| x + y);
  let f2r   = new_function!(&|x:i32,y:i32| x + y);

  let f1p   = Function::F1R(Function1R::new(|s:&String| s ));

  /*
  let f2 = new_function!(|x:i32,y:i32| x + y);
  let f3 = new_function!(|x:&i32| (*x) + 1);

  let mut v: Vec<Box<dyn for<'a> FunctionTrait<'a>>> = Vec::new();


rust dynamic type cast  v.push(unsafe { mem::transmute(f3.inner) });

  let f3 = Function::new(unsafe { mem::transmute(v.pop().unwrap()) });
  */

  println!("{:?}", <&str>::from_value(f0()));
  println!("{:?}", i32::from_value(f1s(1)));
  println!("{:?}", i32::from_value(f1r(&2)));
  println!("{:?}", i32::from_value(f2s(1,2)));
  println!("{:?}", i32::from_value(f2r(&4,&6)));
  println!("{:?}", String::from_value_ref(&f1p&(&s,)));
  println!("{:?}", String::from_value(hello()));
  println!("{:?}", i32::from_value(hello(3)));
  println!("{:?}", i32::from_value(hello(&3)));
  */
}

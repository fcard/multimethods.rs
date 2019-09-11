#![feature(fn_traits)]
#![feature(specialization)]
#![feature(unboxed_closures)]
#![feature(box_syntax)]
#![feature(trait_alias)]
#![feature(decl_macro)]
#![feature(associated_type_defaults)]
#![feature(proc_macro_hygiene)]
#![allow(non_upper_case_globals)]
#![feature(test)]

extern crate test;

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

pub mod conversion;
pub use conversion::*;

use multimethods_proc::*;

#[__fmc]
multifunction! {
  fn hi() -> String {
    format!("Hi, World!")
  }
}

#[__fmc]
multimethods! {
  key=HI;

  fn hi(x: String) -> String {
    format!("Hi, {}!", x)
  }
}

#[__fmc]
multifunction! {
  fn hello() -> String {
    "Hello, World!".to_string()
  }

  fn hello(a: Abstract![INTEGER]) -> String {
    format!("INTEGER: {:?}", a)
  }

  fn hello(a: i32) -> i32 {
    a + 1
  }

  fn hello(a: i32, b: i32) -> i32 {
    a + b
  }

  fn hello(a: &i32) -> i32 {
    a + 2
  }

  fn hello(a: &'static str) {
    println!("{}", a)
  }

  fn hello(s: &String) -> String {
    let mut s2 = s.clone();
    s2.push_str(" and then...");
    s2
  }

  fn hello(s: &String) -> &String {
    s
  }

  fn hello(s: String) {
    println!("{}", s);
  }

  fn hello(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    (a + b + c + d + e)
  }
}

#[__fmc]
multifunction! {
  fn sunga(a: i32) -> i32 {
    a + 1
  }
}

#[__fmc]
multifunction! {
  fn debugin(a: i32) -> String {
    format!("{:?}", a)
  }

  fn debugin(a: &i32) -> String {
    format!("{:?}", a)
  }
}

#[bench]
fn bench_hello(b: &mut Bencher) {
  b.iter(|| {
    hello(1i32, 2i32)
  });
}

#[bench]
fn bench_debug(b: &mut Bencher) {
  b.iter(|| {
    debug(1i32)
  });
}

#[inline(never)]
fn add_i32(a: i32, b: i32) -> i32 {
  a + b
}

#[bench]
fn bench_format(b: &mut Bencher) {
  let a: Box<dyn fmt::Debug> = box 1i32;
  b.iter(|| {
    format!("{:?}", a)
  });
}

#[bench]
fn bench_add(b: &mut Bencher) {
  let x = test::black_box(1);
  let y = test::black_box(2);
  b.iter(|| {
    add_i32(x, y)
  });
}

fn main() {
  initialize_methods(&HI);

  let a0: i32 = 1;
  let s = String::from("hello world!");

  let rr = &a0;
  let kk = rr.into_value_ref();
  println!("{:?}", debug(kk));

  println!("{:?}", hello(1i64));
  println!("{:?}", hello(&s));
  println!("{:?}", (hello.rr)(&s));
  println!("{}", hello(&s) == hello(&s));
  println!("{}", eq(&s, &s));
  println!("{:?}", (1.into_value()) == (2.into_value()));
  println!("{}", eq(hello(&s).into_value_ref(), hello(&s).into_value_ref()));
  println!("{}", (hello.rr)(&s) == (hello.rr)(&s));
  println!("{}", into(Type![i64], 2i32));
  println!("hi0: {}", hi());
  println!("hi1: {}", hi(String::from("john")));

  let mut k = 0i32.into_value();
  for _ in 0..=200_000 {
    k = sunga(k);
  }
  println!("{}", k);
}

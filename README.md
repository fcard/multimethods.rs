# multimethods.rs

Experimental implementation of Multimethods/Multiple Dispatch in Rust, or the ability to overload/dispatch on the runtime type of multiple arguments. This crate is heavily inspired by the [Julia programming language](https://julialang.org), and makes use of the dynamic typing capabilities of Rust as given by the [`Any` trait](https://doc.rust-lang.org/std/any/). Due to the limitation of the latter to `'static` types, the usability and power of this tool have hit an unsastifactory ceilling for the time being.

The crate's performance hasn't been thoroughly tested but it can be assumed to be very, very slow, (unless Rust has done some sort of miracle work) as it was meant primarily as personal research, and tricky optimizations in general have been avoided.

Nevertheless, contributions are very welcome!

## How To Use

To define a new function, use the `multifunction!` macro, like so:
```rust
use multimethods::multifunction;

// only one function may be defined here (for now)
multifuntion! {
  pub fn HELLO() {
    println!("Hello, World!");
  }

  pub fn HELLO(x: &'static str) {
    println!("Hello, {}!", x);
  }

  pub fn HELLO(x: String) {
    println!("Hello, Stringified {}!", x);
  }
}

fn main() {
  HELLO();                   // Hello, World!
  HELLO("John");             // Hello, John!
  HELLO("John".to_string()); // Hello, Stringified John!
}
```

Adding new methods to existing multi-functions requires the use of `multimethods!` and the specification of a named key.

```rust
use multimethods::{multifunction, multimethods, initialize_methods};

// Functions with no methods can be defined by passing a name to multifunction!
multifunction!(HI);

// multiple functions can be extended here
// note that only functions defined with multifunction! can be extended
multimethods! {
  key=NUMERIC;

  pub fn HI(x: i32) {
    println!("Hi, Number {}!", x);
  }

  pub fn HELLO(x: i32) { // Assuming HELLO has already been defined and is in scope
    println!("Hello, Number {}!", x);
  }
}

fn main() {
  initialize_methods(&NUMERIC); // Without this, a "Method Not Found" panic would occur bellow

  HELLO(1); // Hello, Number 1!
  HI(2);    // HI, Number 2!
}
```

Currently, all the arguments of a method must either be references or values, e.g.
```rust
multifunction! {
  // valid
  fn F(x: i32, y: i32) -> i32 {
    x + y
  }

  // valid
  fn F(x: &i32, y: &i32) -> i32 {
    x + y
  }

  // NOT valid!
  fn F(x: &i32, y: i32) -> i32 {
    x + y
  }
}
```

Return values can vary between methods, but note that they will always be converted to one of `multimethods::value::Value` or `multimethods::value_ref::ValueRef<'_>`. Use `<T>::from_value(x)` to convert a `x: Value` to `T` or `<T>::from_value_ref(x)` to convert a `x: ValueRef<'a>` to a `&'a T` (for now, this will panic if `x` cannot be converted)

```rust
use multimethods::{multifunction, FromValue};

multifunction! {
  fn ADD_INT(x: i32, y: i32) -> i32 {
    x + y
  }

  fn ADD_INT(x: i64, y: i64) -> i64 {
    x + y
  }
}

fn main() {
  println!("{}", i32::from_value(ADD_INT(1i32, 1i32))); // 2
  println!("{}", i64::from_value(ADD_INT(1i64, 1i64))); // 2

  // this will currently panic: i32::from_value(ADD_INT(1i64, 1i64))
}
```

If the `traits` feature is enabled (true by default), then some of the more common traits are implemented for the `Value` type, in terms of multi-functions defined for the more common types.

```rust
use multimethods::{multifunction, multimethods, initialize_methods, debug, clone, IntoValue};

#[derive(Clone, Copy, Debug)]
struct MyType(i32);

multifunction! {
  pub fn ADD_INT(x: i32, y: i32) -> i32 {
    x + y
  }
}


// Debug:
// Implementing the Debug trait for a `Value` containing something of type T
// requires a method of the `multimethods::traits::debug` multi-function that
// takes a &T and returns a String.

// Clone:
// Implementing the Clone trait for a `Value` containing something of type T
// requires a method of the `multimethods::traits::clone` multi-function that
// takes a &T and returns a T.

multimethods! {
  key=MY_TYPE_TRAITS;

  pub fn debug(x: &MyType) -> String {
    format!("{:?}", x)
  }

  pub fn clone(x: &MyType) -> MyType {
    x.clone()
  }
}


fn main() {
  // works out of the box
  println!("{}", ADD_INT(1,2) + 3); // 6

  // not implementing these methods would result in a "Method Not Found" panic
  initialize_methods(&MY_TYPE_TRAITS);
  println!("{:?}", MyType(1).into_value().clone()); // MyType(1)
}
```

Methods that return references requires the call to be expressed as `(FUNC.rr)(args...)`. Hopefully this won't be necessary in the future. (or I will find a nicer syntax, at least)

```rust
multifunction! {
  pub fn SELF(x: String) -> String {
    x
  }

  pub fn SELF(x: &String) -> &String {
    x
  }
}

fn main() {
 let a = "a".to_string();
 let b = "b".to_string();

  println!("{}", SELF(a));       // a
  println!("{}", (SELF.rr)(&b)); // b
}
```

Generics are not implemented. Instead, for generic programming, you must use Julia-styled abstract types.

```rust
use multimethods::{multifunction, Abstract, ANY, NUMBER};

multifunction! {
  pub fn DESCRIBE(_x: Abstract![ANY]) {
    // inside the function, the actual type of _x is Value;
    // this is true for any abstract type
    println!("I am of any type!")
  }

  pub fn DESCRIBE(_x: Abstract![NUMBER]) {
    println!("I am of some numeric type!")
  }

  pub fn DESCRIBE(x: i32) {
    println!("I am a i32 of value {}!", x)
  }
}

fn main() {
  DESCRIBE("a");  // I am of any type!
  DESCRIBE(1.0);  // I am of some numeric type!
  DESCRIBE(1i32); // I am a i32 of value 1!
}
```

To define a new type, use `new_abstract_type!`, and to implement it for concrete types, use `impl_abstract_type!`. Note that types can only have one abstract parent.

```rust
#![feature(specialization)] // required to subtype abstract types

use multimethods::{multifunction, new_abstract_type, impl_abstract_type, Abstract, ANY};

struct MyType1(i32);
struct MyType2(String);
struct MyType3(Value);

new_abstract_type! {
  MY_ABSTRACT1, // supertype defaults to ANY
  MY_ABSTRACT2: MY_ABSTRACT1,
}

impl_abstract_type! {
  MyType1: MY_ABSTRACT1,
  MyType2: MY_ABSTRACT2,
  MyType3: MY_ABSTRACT2
}

multifunction! {
  pub fn DESCRIBE(_x: Abstract![ANY]) {
    println!("any")
  }

  pub fn DESCRIBE(_x: Abstract![MY_ABSTRACT1]) {
    println!("abstract 1")
  }

  pub fn DESCRIBE(_x: Abstract![MY_ABSTRACT2]) {
    println!("abstract 2")
  }

  pub fn DESCRIBE(_x: MyType3) {
    println!("my type 3")
  }
}

fn main() {
  DESCRIBE("a");                       // any
  DESCRIBE(MyType1(1));                // abstract 1
  DESCRIBE(MyType2("a".to_string()));  // abstract 2
  DESCRIBE(MyType3("a".into_value())); // my type 3
}
```

A more powerful way to implement an abstract type is to directly `impl` the `SubType` trait:

```rust
#![feature(specialization)]
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

  pub fn IS_MY_COLLECTION(_: Abstract![COLLECTION]) -> bool {
    true
  }
}

fn main() {
  let coll1 = MyCollection(vec![1,2,3]);
  let coll2 = MyCollection(vec!["a","b"]);

  println!("{}", IS_MY_COLLECTION(1));     // false
  println!("{}", IS_MY_COLLECTION(coll1)); // true
  println!("{}", IS_MY_COLLECTION(coll2)); // true
}
```

A variadic method can be defined using the special `Vararg![T]` macro. The type of the variadic
argument is `multimethods::types::vararg::Vararg<T>`, which can be iterated through and indexed.

```rust
// Vararg doesn't need to be imported as it's merely a marker for the multifunction! macro
use multimethods::multifunction;

multifunction! {
  fn SUM(args: Vararg![i32]) -> i32 {
    args.iter().sum()
  }
}

// Vararg![] is equivalent to Vararg![Abstract![ANY]]
multifunction! {
  fn PRINT_ALL(args: Vararg![])  {
    for arg in args {
      println!("{}", arg)
    }
  }
}

fn main() {
  println!("{}", SUM(1, 2, 3)); // 6

  PRINT_ALL("a", 2); // a
                     // 2
}
```


## Limitations

* Only up to 12 arguments per method are allowed. This number was chosen as it is the largest size of a tuple that has trait implementations for it in the standard library.

* As previously said, reference and static arguments cannot be mixed. There is also no support for mutable references, currently.

* All non-reference types must currently be `'static`.

* A relatively minor issue, but function names with lowercase letters give a warning if `#[warn(non_upper_case_globals)]` is on.
  (See [lazy-static.rs#153](https://github.com/rust-lang-nursery/lazy-static.rs/issues/153#issue-478689023))


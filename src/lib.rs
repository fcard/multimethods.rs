#![feature(fn_traits)]
#![feature(specialization)]
#![feature(unboxed_closures)]
#![feature(box_syntax)]
#![feature(trait_alias)]
#![feature(decl_macro)]
#![feature(associated_type_defaults)]
#![feature(never_type)]
#![allow(non_upper_case_globals)]
#![feature(test)]

pub use lazy_static;

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

pub use multimethods_proc::*;

#[cfg(feature = "traits")]
pub mod traits;

#[cfg(feature = "traits")]
pub use traits::*;


#[cfg(feature = "conversion")]
pub mod conversion;

#[cfg(feature = "conversion")]
pub use conversion::*;


#[cfg(feature = "type_of")]
pub mod type_of;

#[cfg(feature = "type_of")]
pub use type_of::*;




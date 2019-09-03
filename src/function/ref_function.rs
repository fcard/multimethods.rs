use crate::value_ref::*;
use crate::function::helper_macros::*;

pub trait RefFn<'a, Args> {
  fn r(&self, a: Args) -> ValueRef<'a>;
}

impl_ref_fn!(F1R, A);
impl_ref_fn!(F2R, A, B);


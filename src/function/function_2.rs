use crate::function::helper_macros::*;

// Types

pub enum Function2 {
  S(fnbox_static!(a,b)),
  R(fnbox_ref!(a,b)),
}

pub struct Function2R {
  pub inner: fnbox_ref_return!(a,b)
}

// Impls

impl_function! {
  Function2(a: A, b: B)
}

impl_ref_function! {
  Function2R(a: A, b: B)
}


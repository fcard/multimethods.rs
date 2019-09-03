use crate::value::*;

pub struct Function0 {
  inner: Box<dyn Fn() -> Value>
}

pub trait NewFunction0 {
  fn new_function(self) -> Function0;
}

impl Function0 {
  pub fn new<R,F>(func: F) -> Self
    where
      R: IntoValue + 'static,
      F: Fn() -> R + 'static,
  {
    Function0 {
      inner: box move || {
        func().into_value()
      }
    }
  }

  pub fn call_once(self, _: ()) -> Value {
    self.inner.call_once(()).into_value()
  }

  pub fn call_mut(&mut self, _: ()) -> Value {
    self.inner.call_mut(()).into_value()
  }

  pub fn call(&self, _: ()) -> Value {
    self.inner.call(()).into_value()
  }
}


use crate::*;

// CLONE

trait ElementClone {
  fn clone_element(&self) -> Element;
}

impl<T: ElementTrait + Clone> ElementClone for T {
  fn clone_element(&self) -> Element {
    Box::new(self.clone())
  }
}

impl Clone for Element {
  fn clone(&self) -> Element {
    self.clone_element()
  }
}


// DEBUG

trait ElementDebug {
  fn debug_element(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<T: ElementTrait + Debug> ElementDebug for T {
  fn debug_element(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Debug for Element {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.debug_element(f)
  }
}

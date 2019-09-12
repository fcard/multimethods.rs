use std::ops::*;
use std::iter::*;

pub struct Vararg<T> {
  elements: Vec<T>
}

pub macro Vararg {
  () => { Abstract![ANY] },
  ($T: ty) => { $T }
}

impl<T> Vararg<T> {
  pub fn new(elements: Vec<T>) -> Self {
    Vararg { elements }
  }

  pub fn iter(&self) -> impl Iterator<Item=&T> {
    self.elements.iter()
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
    self.elements.iter_mut()
  }
}

impl<'a, T> IntoIterator for &'a Vararg<T> {
  type Item = &'a T;
  type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    (&self.elements).into_iter()
  }
}

impl<'a, T> IntoIterator for &'a mut Vararg<T> {
  type Item = &'a mut T;
  type IntoIter = <&'a mut Vec<T> as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    (&mut self.elements).into_iter()
  }
}

impl<'a, T> IntoIterator for Vararg<T> {
  type Item = T;
  type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

  fn into_iter(self) -> Self::IntoIter {
    self.elements.into_iter()
  }
}

impl<I,T> Index<I> for Vararg<T>
  where
    I: std::slice::SliceIndex<[T]>
{
  type Output = <Vec<T> as Index<I>>::Output;

  fn index(&self, index: I) -> &Self::Output {
    &self.elements[index]
  }
}

impl<I,T> IndexMut<I> for Vararg<T>
  where
    I: std::slice::SliceIndex<[T]>
{
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    &mut self.elements[index]
  }
}


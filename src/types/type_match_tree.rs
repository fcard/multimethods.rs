use crate::types::*;

pub struct TypeMatchNode<T> {
  value: T,
  type_match: TypeMatchTuple,
  children: Vec<TypeMatchNode<T>>
}

pub struct TypeMatchTree<T> {
  children: Vec<TypeMatchNode<T>>,
  children_rr: Vec<TypeMatchNode<T>>
}

impl<T> TypeMatchTree<T> {
  pub fn new() -> Self {
    TypeMatchTree {
      children: Vec::new(),
      children_rr: Vec::new(),
    }
  }

  fn children(&self, rr: bool) -> &Vec<TypeMatchNode<T> > {
    if rr { &self.children_rr } else { &self.children }
  }

  fn children_mut(&mut self, rr: bool) -> &mut Vec<TypeMatchNode<T> > {
    if rr { &mut self.children_rr } else { &mut self.children }
  }

  pub fn insert(&mut self, key: TypeMatchTuple, value: T, rr: bool) {
    insert_to_children(self.children_mut(rr), key, value);
  }

  pub fn get<'a>(&'a self, key: &TypeMatchTuple, rr: bool) -> Option<&'a T> {
    get_from_children(self.children(rr), key)
  }

  pub fn get_mut<'a>(&'a mut self, key: &TypeMatchTuple, rr: bool) -> Option<&'a mut T> {
    get_from_children_mut(self.children_mut(rr), key)
  }

  pub fn remove<'a>(&'a mut self, key: &TypeMatchTuple, rr: bool) -> Option<T> {
    remove_from_children(self.children_mut(rr), key)
  }
}

impl<T> TypeMatchNode<T> {
  fn new(key: TypeMatchTuple, value: T) -> Self {
    TypeMatchNode {
      value,
      type_match: key,
      children: Vec::new()
    }
  }

  fn try_insert(&mut self, key: TypeMatchTuple, value: T) -> Option<(TypeMatchTuple, T)> {
    if key == self.type_match {
      panic!("method overwritten");

    } else if matches_all(&self.type_match, &key) {
      insert_to_children(&mut self.children, key, value);
      None

    } else {
      Some((key, value))
    }
  }
}


fn insert_to_children<T>(
  children: &mut Vec<TypeMatchNode<T>>, mut key: TypeMatchTuple, mut value: T) {

  for child in children.iter_mut() {
    if let Some((key_, value_)) = child.try_insert(key, value) {
      key   = key_;
      value = value_;

    } else {
      return;
    }
  }
  children.push(TypeMatchNode::new(key, value));
}


fn get_from_children<'a, T>(
  children: &'a Vec<TypeMatchNode<T>>, key: &TypeMatchTuple) -> Option<&'a T> {

  for child in children.iter() {
    if matches_all(&child.type_match, key) {
      if let Some(value) = get_from_children(&child.children, key) {
        return Some(value);

      } else {
        return Some(&child.value);
      }
    }
  }
  return None;
}

fn get_from_children_mut<'a, T>(
  children: &'a mut Vec<TypeMatchNode<T>>, key: &TypeMatchTuple) -> Option<&'a mut T> {

  for child in children.iter_mut() {
    if matches_all(&child.type_match, key) {
      if let Some(value) = get_from_children_mut(&mut child.children, key) {
        return Some(value)

      } else {
        return Some(&mut child.value)
      }
    }
  }
  return None;
}

fn remove_from_children<'a, T> (
  children: &'a mut Vec<TypeMatchNode<T>>, key: &TypeMatchTuple) -> Option<T> {

  for i in 0..children.len() {
    if matches_all(&children[i].type_match, key) {
      if let Some(removed) = remove_from_children(&mut children[i].children, key) {
        return Some(removed);

      } else {
        let mut m = children.remove(i);
        children.append(&mut m.children);
        return Some(m.value);
      }
    }
  }
  None
}


use crate::types::*;

pub enum MatchValue<T> {
  Single(T),
  Vararg(usize, Vec<T>)
}

pub struct TypeMatchNode<T> {
  value: MatchValue<T>,
  type_match: TypeMatches,
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

  pub fn insert(&mut self, key: TypeMatches, value: T, rr: bool) {
    insert_to_children(self.children_mut(rr), TypeMatchNode::new(key, value));
  }

  pub fn insert_vararg(&mut self, key: TypeMatches, p: usize, values: Vec<T>, rr: bool) {
    insert_to_children(self.children_mut(rr), TypeMatchNode::new_vararg(key, p, values));
  }

  pub fn get<'a>(&'a self, key: &TypeMatches, rr: bool) -> Option<&'a T> {
    get_from_children(self.children(rr), key)
  }

  pub fn get_mut<'a>(&'a mut self, key: &TypeMatches, rr: bool) -> Option<&'a mut T> {
    get_from_children_mut(self.children_mut(rr), key)
  }

  pub fn remove<'a>(&'a mut self, key: &TypeMatches, rr: bool) -> Option<T> {
    remove_from_children(self.children_mut(rr), key)
  }
}

impl<T> TypeMatchNode<T> {
  fn new(key: TypeMatches, value: T) -> Self {
    TypeMatchNode {
      value: MatchValue::Single(value),
      type_match: key,
      children: Vec::new()
    }
  }

  fn new_vararg(key: TypeMatches, positionals: usize, values: Vec<T>) -> Self {
    TypeMatchNode {
      value: MatchValue::Vararg(positionals, values),
      type_match: key,
      children: Vec::new()
    }
  }
}

impl<T> MatchValue<T> {
  fn get<'a>(&'a self, key: &TypeMatches) -> &'a T {
    match self {
      MatchValue::Single(value) => value,
      MatchValue::Vararg(n,values) => {
        &values[key.len() - n]
      }
    }
  }

  fn get_mut<'a>(&'a mut self, key: &TypeMatches) -> &'a mut T {
    match self {
      MatchValue::Single(value) => value,
      MatchValue::Vararg(n,values) => {
        &mut values[key.len() - *n]
      }
    }
  }

  fn remove(self, key: &TypeMatches) -> T {
    match self {
      MatchValue::Single(value) => value,
      MatchValue::Vararg(n, mut values) => {
        values.remove(key.len() - n)
      }
    }
  }
}


fn insert_to_children<T>(
  children: &mut Vec<TypeMatchNode<T>>, mut node: TypeMatchNode<T>) {

  let mut swipe  = Vec::new();
  let mut add_to = None;

  for i in 0..children.len() {
    if node.type_match == children[i].type_match {
      panic!("Method Overwritten.");

    } else if matches_all(&children[i].type_match, &node.type_match) {
      if let Some(_) = add_to {
        panic!("Ambiguous Method.");

      } else {
        add_to = Some(i);
      }
    } else if matches_all(&node.type_match, &children[i].type_match) {
      swipe.push(i);
    }
  }

  for s in swipe.iter().rev() {
    insert_to_children(&mut node.children, children.remove(*s));
  }

  if let Some(mut i) = add_to {
    i -= swipe.iter().filter(|s| **s < i).count();
    insert_to_children(&mut children[i].children, node)

  } else {
    children.push(node);
  }
}


fn get_from_children<'a, T>(
  children: &'a Vec<TypeMatchNode<T>>, key: &TypeMatches) -> Option<&'a T> {

  for child in children.iter() {
    if matches_all(&child.type_match, key) {
      if let Some(value) = get_from_children(&child.children, key) {
        return Some(value);

      } else {
        return Some(child.value.get(key));
      }
    }
  }
  return None;
}

fn get_from_children_mut<'a, T>(
  children: &'a mut Vec<TypeMatchNode<T>>, key: &TypeMatches) -> Option<&'a mut T> {

  for child in children.iter_mut() {
    if matches_all(&child.type_match, key) {
      if let Some(value) = get_from_children_mut(&mut child.children, key) {
        return Some(value)

      } else {
        return Some(child.value.get_mut(key))
      }
    }
  }
  return None;
}

fn remove_from_children<'a, T> (
  children: &'a mut Vec<TypeMatchNode<T>>, key: &TypeMatches) -> Option<T> {

  for i in 0..children.len() {
    if matches_all(&children[i].type_match, key) {
      if let Some(removed) = remove_from_children(&mut children[i].children, key) {
        return Some(removed);

      } else {
        let mut m = children.remove(i);
        children.append(&mut m.children);
        return Some(m.value.remove(key));
      }
    }
  }
  None
}


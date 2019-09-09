struct AbstractType {
  parent: Option<Box<AbstractType>>
}

impl AbstractType {
  type P<'a> = &'a dyn Any;
}

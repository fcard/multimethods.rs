use std::marker::PhantomData;

#[derive(PartialEq,Clone,Copy)]
pub struct Type<T>(PhantomData<T>);

pub fn _type<T>() -> Type<T> {
  Type(PhantomData)
}

pub macro Type($T: ty) {
  _type::<$T>()
}


pub macro new_function {
  // Zero arguments

  //-- Owned Arguments

  (|| $body: expr) => {
    Function::F0(Function0::new(||$body))
  },

  (|| -> $R: ty  { $body: expr }) => {
    Function::F0(Function0::new(|| -> $R { $body }))
  },

  //-- Reference Arguments

  (| | $body: expr) => {
    Function::F0(Function0::new(||$body))
  },

  (| | -> $R: ty  { $body: expr }) => {
    Function::F0(Function0::new(|| -> $R { $body }))
  },


  // One Argument

  //-- Owned Arguments

  (|$a: tt$(: $A: ty)?| $body: expr) => {
    Function::F1(
      Function1::new_s(
        |$a$(: $A)*| $body
      )
    )
  },

  (|$a: tt$(: $A: ty)?| -> $R: ty { $body: expr }) => {
    Function::F1(
      Function1::new_s(
        |$a$(: $A)*| -> $R { $body }
      )
    )
  },

  //-- Reference Arguments

  (&|$a: tt$(: $A: ty)?| $body: expr) => {
    Function::F1(
      Function1::new_r(
        |$a$(: &$A)*| $body
      )
    )
  },

  (&|$a: tt$(: $A: ty)?| -> $R: ty { $body: expr }) => {
    Function::F1(
      Function1::new_r(
        |$a$(: &$A)*| -> $R { $body }
      )
    )
  },

  //-- Reference Arguments and Return value

  (&&|$a: tt$(: $A: ty)?| -> $R: ty { $body: expr }) => {
    Function::F1R(
      Function1R::new(
        |$a$(: &$A)*| -> &$R { $body }
      )
    )
  },


  // Two Arguments

  //-- Owned Arguments

  (|$a: ident$(: $A: ty)?, $b: ident$(: $B: ty)?| $body: expr) => {
    Function::F2(
      Function2::new_s(
        |$a$(: $A)*,$b$(: $B)*| $body
      )
    )
  },

  (|$a: ident$(: $A: ty)?, $b: ident$(: $B: ty)?| -> $R: ty { $body: expr }) => {
    Function::F2(
      Function2::new_s(
        |$a$(: $A)*,$b$(: $B)*| -> $R { $body }
      )
    )
  },

  //-- Reference Arguments

  (&|$a: ident$(: $A: ty)?, $b: ident$(: $B: ty)?| $body: expr) => {
    Function::F2(
      Function2::new_r(
        |$a$(: &$A)*,$b$(: &$B)*| $body
      )
    )
  },

  (&|$a: ident$(: $A: ty)?, $b: ident$(: $B: ty)?| -> $R: ty { $body: expr }) => {
    Function::F2(
      Function2::new_r(
        |$a$(: &$A)*,$b$(: &$B)*| -> $R { $body }
      )
    )
  },

  //-- Reference Arguments and Return value

  (&&|$a: ident$(: $A: ty)?, $b: ident$(: $B: ty)?| -> $R: ty { $body: expr }) => {
    Function::F2R(
      Function2R::new(
        |$a$(: &$A)*,$b$(: &$B)*| -> &$R { $body }
      )
    )
  }
}


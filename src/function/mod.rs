pub mod function_0;
pub mod function_n;

pub mod inner_function;
pub mod helper_macros;
pub mod ref_function;
pub mod new_function;
pub mod call_0;
pub mod call_n;

pub use function_0::*;
pub use function_n::*;

pub use inner_function::*;
pub use ref_function::*;
pub use new_function::*;
pub use call_0::*;
pub use call_n::*;

pub enum Function {
  F0(Function0),

  F1(Function1),
  F1R(Function1R),

  F2(Function2),
  F2R(Function2R),

  F3(Function3),
  F3R(Function3R),

  F4(Function4),
  F4R(Function4R),

  F5(Function5),
  F5R(Function5R),

  F6(Function6),
  F6R(Function6R),

  F7(Function7),
  F7R(Function7R),

  F8(Function8),
  F8R(Function8R),

  F9(Function9),
  F9R(Function9R),

  F10(Function10),
  F10R(Function10R),

  F11(Function11),
  F11R(Function11R),

  F12(Function12),
  F12R(Function12R),

  F13(Function13),
  F13R(Function13R),

  F14(Function14),
  F14R(Function14R),

  F15(Function15),
  F15R(Function15R),

  F16(Function16),
  F16R(Function16R),
}


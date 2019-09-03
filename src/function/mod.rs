pub mod function_0;
pub mod function_1;
pub mod function_2;

pub mod helper_macros;
pub mod ref_function;
pub mod new_function;
pub mod call_0;
pub mod call_n;
pub mod inner_function;

pub use function_0::*;
pub use function_1::*;
pub use function_2::*;

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
}


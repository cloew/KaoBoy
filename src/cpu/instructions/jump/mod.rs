pub mod instructions;
mod conditions;
mod jump;
mod jump_instruction;

pub use conditions::{always, is_carry_flag_off, is_carry_flag_on, is_zero_flag_off, is_zero_flag_on};
pub use jump::{JumpConditionFn, jump, jump_with_extra_work};
pub mod instructions;
mod conditions;
mod jump;
mod jump_instruction;

pub use jump::{JumpConditionFn, jump};
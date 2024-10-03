mod codegen;
mod codegen_verb;
mod codegen_mc;

use data::*;
pub use codegen::{codegen, operands, Operands, Instruction, nasm};
pub use codegen_verb::codegen_verb;

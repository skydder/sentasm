mod codegen;
mod codegen_verb;
mod codegen_mc;

use data::*;
pub use codegen::{codegen, Operands, Instruction, nasm};
pub use codegen_verb::codegen_verb;
pub use codegen_mc::emit_mc;
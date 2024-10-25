mod codegen;
mod codegen_verb;
mod codegen_mc;
mod prep_x86_64;

use data::*;
pub use codegen::codegen;
pub(crate) use codegen_verb::codegen_verb;
pub(crate) use codegen_mc::emit_mc;
pub(crate) use prep_x86_64::{Instruction, Operands, nasm};
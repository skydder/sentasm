mod bits;
mod mod_rm;
mod intstruction;
mod rex;
mod sib;
mod vex;
mod evex;

pub use intstruction::{nasm, Instruction, Operands};
pub(crate) use bits::Bits;
pub(crate) use mod_rm::ModRM;
pub(crate) use rex::Rex;
pub(crate) use sib::SIB;
// pub(crate) use evex::Evex
// pub(crate) use vex::Vex;
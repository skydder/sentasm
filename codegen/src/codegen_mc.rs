use data::{
	Register, Immediate, Memory, Data, DataSet
};
use crate::{Operands, Instruction};
use macros::match_data;

pub fn emit_mc<'a>(ins_name: &str, operands: Operands<'a>) -> Result<Instruction, Operands<'a>> {
	let mut ins = Instruction::new();
	match (ins_name, &operands) {
		("adc", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x11);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("adc", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x11);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("adc", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x13);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("adc", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x13);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("adc", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			ins.set_imm(_i);
			Ok(ins)
		},
		("adc", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			ins.set_imm(_i);
			Ok(ins)
		},
		("adc", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(_, _, ..)), None, None)) => {
			// -i
			// [[], ['i']]
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x15);
			ins.set_imm(_i);
			Ok(ins)
		},
		("adc", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			ins.set_imm(_i);
			Ok(ins)
		},
		("adc", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, _, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x81);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			ins.set_imm(_i);
			Ok(ins)
		},
		("add", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x01);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("add", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x01);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("add", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x03);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("add", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x03);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("add", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			ins.set_imm(_i);
			Ok(ins)
		},
		("add", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			ins.set_imm(_i);
			Ok(ins)
		},
		("add", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(_, _, ..)), None, None)) => {
			// -i
			// [[], ['i']]
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x05);
			ins.set_imm(_i);
			Ok(ins)
		},
		("add", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			ins.set_imm(_i);
			Ok(ins)
		},
		("add", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, _, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x81);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			ins.set_imm(_i);
			Ok(ins)
		},
		("and", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x21);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("and", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x21);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("and", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x23);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("and", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x23);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("and", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			ins.set_imm(_i);
			Ok(ins)
		},
		("and", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			ins.set_imm(_i);
			Ok(ins)
		},
		("and", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(_, _, ..)), None, None)) => {
			// -i
			// [[], ['i']]
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x25);
			ins.set_imm(_i);
			Ok(ins)
		},
		("and", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			ins.set_imm(_i);
			Ok(ins)
		},
		("and", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, _, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x81);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			ins.set_imm(_i);
			Ok(ins)
		},
		("bsf", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			// unimplemented
			ins.set_opcode(0x0f);
			ins.set_opcode(0xbc);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("bsf", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			// unimplemented
			ins.set_opcode(0x0f);
			ins.set_opcode(0xbc);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("bsr", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			// unimplemented
			ins.set_opcode(0x0f);
			ins.set_opcode(0xbd);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("bsr", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			// unimplemented
			ins.set_opcode(0x0f);
			ins.set_opcode(0xbd);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("bswap", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// r
			// [['r']]
			let _r = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opecode_with_register(0xc8, _r);
			Ok(ins)
		},
		("bt", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xa3);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("bt", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xa3);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("bt", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xba);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			ins.set_imm(_i);
			Ok(ins)
		},
		("btc", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xbb);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("btc", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xbb);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("btc", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xba);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			ins.set_imm(_i);
			Ok(ins)
		},
		("btr", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xb3);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("btr", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xb3);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("btr", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xba);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			ins.set_imm(_i);
			Ok(ins)
		},
		("bts", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xab);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("bts", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xab);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("bts", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xba);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			ins.set_imm(_i);
			Ok(ins)
		},
		("call", Operands(match_data!(Immediate(_, 64, ..)), None, None, None)) => {
			// i
			// [['i']]
			let _i = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0xe8);
			ins.set_disp(_i);
			Ok(ins)
		},
		("call", Operands(match_data!(Immediate(_, _, ..)), None, None, None)) => {
			// i
			// [['i']]
			let _i = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0xe8);
			ins.set_disp(_i);
			Ok(ins)
		},
		("call", Operands(match_data!(Memory{size:_, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xff);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			Ok(ins)
		},
		("call", Operands(match_data!(Memory{size:_, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xff);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			Ok(ins)
		},
		("call", Operands(match_data!(Register(_, _, _, ..)) | match_data!(Memory{size:_, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0xff);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			Ok(ins)
		},
		("call", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0xff);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			Ok(ins)
		},
		("cdqe", Operands(None, None, None, None)) => {
			// void
			ins.set_rex();
			ins.set_opcode(0x98);
			Ok(ins)
		},
		("cmp", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x39);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("cmp", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x39);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("cmp", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x3b);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("cmp", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x3b);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("cmp", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			ins.set_imm(_i);
			Ok(ins)
		},
		("cmp", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			ins.set_imm(_i);
			Ok(ins)
		},
		("cmp", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(_, _, ..)), None, None)) => {
			// -i
			// [[], ['i']]
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x3d);
			ins.set_imm(_i);
			Ok(ins)
		},
		("cmp", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			ins.set_imm(_i);
			Ok(ins)
		},
		("cmp", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, _, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x81);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			ins.set_imm(_i);
			Ok(ins)
		},
		("cmpsq", Operands(None, None, None, None)) => {
			// void
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0xa7);
			Ok(ins)
		},
		("cmpxchg", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xb1);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("cmpxchg", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xb1);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("cmpxchg16b", Operands(match_data!(Memory{size:128, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xc7);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			Ok(ins)
		},
		("cqo", Operands(None, None, None, None)) => {
			// void
			ins.set_rex();
			ins.set_opcode(0x99);
			Ok(ins)
		},
		("dec", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0xff);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			Ok(ins)
		},
		("div", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xf7);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			Ok(ins)
		},
		("idiv", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xf7);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			Ok(ins)
		},
		("imul", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xf7);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			Ok(ins)
		},
		("imul", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xaf);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("imul", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xaf);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("imul", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), match_data!(Immediate(_, 8, ..)), None)) => {
			// rmi
			// [['r'], ['m'], ['i']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x6b);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("imul", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), match_data!(Immediate(..)), None)) => {
			// rmi
			// [['r'], ['m'], ['i']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x6b);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("imul", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), match_data!(Immediate(_, 32, ..)), None)) => {
			// rmi
			// [['r'], ['m'], ['i']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x69);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("imul", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), match_data!(Immediate(_, _, ..)), None)) => {
			// rmi
			// [['r'], ['m'], ['i']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x69);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("imul", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), match_data!(Immediate(_, 8, ..)), None)) => {
			// rmi
			// [['r'], ['m'], ['i']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x6b);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("imul", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), match_data!(Immediate(..)), None)) => {
			// rmi
			// [['r'], ['m'], ['i']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x6b);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("imul", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), match_data!(Immediate(_, 32, ..)), None)) => {
			// rmi
			// [['r'], ['m'], ['i']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x69);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("imul", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), match_data!(Immediate(_, _, ..)), None)) => {
			// rmi
			// [['r'], ['m'], ['i']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x69);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		// ("imul", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Immediate(_, 8, ..)), None, None)) => {
		// 	// r+mi
		// 	// [['r', 'm'], ['i']]
		// 	let _r = operands.0.unwrap();
		// 	let _m = operands.0.unwrap();
		// 	let _i = operands.1.unwrap();
		// 	ins.set_rex();
		// 	ins.set_opcode(0x6b);
		// 	ins.set_rm(_m);
		// 	ins.set_reg(_r);
		// 	ins.set_imm(_i);
		// 	Ok(ins)
		// },
		// ("imul", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Immediate(..)), None, None)) => {
		// 	// r+mi
		// 	// [['r', 'm'], ['i']]
		// 	let _r = operands.0.unwrap();
		// 	let _m = operands.0.unwrap();
		// 	let _i = operands.1.unwrap();
		// 	ins.set_rex();
		// 	ins.set_opcode(0x6b);
		// 	ins.set_rm(_m);
		// 	ins.set_reg(_r);
		// 	ins.set_imm(_i);
		// 	Ok(ins)
		// },
		// ("imul", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Immediate(_, 32, ..)), None, None)) => {
		// 	// r+mi
		// 	// [['r', 'm'], ['i']]
		// 	let _r = operands.0.unwrap();
		// 	let _m = operands.0.unwrap();
		// 	let _i = operands.1.unwrap();
		// 	ins.set_rex();
		// 	ins.set_opcode(0x69);
		// 	ins.set_rm(_m);
		// 	ins.set_reg(_r);
		// 	ins.set_imm(_i);
		// 	Ok(ins)
		// },
		// ("imul", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Immediate(_, _, ..)), None, None)) => {
		// 	// r+mi
		// 	// [['r', 'm'], ['i']]
		// 	let _r = operands.0.unwrap();
		// 	let _m = operands.0.unwrap();
		// 	let _i = operands.1.unwrap();
		// 	ins.set_rex();
		// 	ins.set_opcode(0x69);
		// 	ins.set_rm(_m);
		// 	ins.set_reg(_r);
		// 	ins.set_imm(_i);
		// 	Ok(ins)
		// },
		("inc", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0xff);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("iretq", Operands(None, None, None, None)) => {
			// void
			ins.set_rex();
			ins.set_opcode(0xcf);
			Ok(ins)
		},
		("jrcxz", Operands(match_data!(Immediate(_, _, ..)), None, None, None)) => {
			// i
			// [['i']]
			let _i = operands.0.unwrap();
			ins.set_rex_w();
			// unimplemented
			ins.set_opcode(0xe3);
			ins.set_disp(_i);
			Ok(ins)
		},
		("jmp", Operands(match_data!(Immediate(_, 64, ..)), None, None, None)) => {
			// i
			// [['i']]
			let _i = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0xe9);
			ins.set_disp(_i);
			Ok(ins)
		},
		("jmp", Operands(match_data!(Immediate(_, _, ..)), None, None, None)) => {
			// i
			// [['i']]
			let _i = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0xe9);
			ins.set_disp(_i);
			Ok(ins)
		},
		("jmp", Operands(match_data!(Memory{size:_, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xff);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			Ok(ins)
		},
		("jmp", Operands(match_data!(Memory{size:_, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xff);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			Ok(ins)
		},
		("jmp", Operands(match_data!(Register(_, _, _, ..)) | match_data!(Memory{size:_, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0xff);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("jmp", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0xff);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("lar", Operands(match_data!(Register(_, _, 16, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_prefix(0x66);
			ins.set_rex_w();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x02);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lar", Operands(match_data!(Register(_, _, 32, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			// 32 bit operand
			ins.set_rex_w();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x02);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lar", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x02);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lar", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 16, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x02);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lar", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 32, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x02);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lar", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x02);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lea", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x8d);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lea", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Immediate(_, _, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x8d);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lfs", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xb4);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lgs", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xb5);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lodsq", Operands(None, None, None, None)) => {
			// void
			ins.set_rex();
			ins.set_opcode(0xad);
			Ok(ins)
		},
		("loop", Operands(match_data!(Immediate(_, _, ..)), match_data!(Register(_, 1, 64, ..)), None, None)) => {
			// i-
			// [['i'], []]
			let _i = operands.0.unwrap();
			// unimplemented
			ins.set_opcode(0xe2);
			ins.set_disp(_i);
			Ok(ins)
		},
		("loope", Operands(match_data!(Immediate(_, _, ..)), match_data!(Register(_, 1, 64, ..)), None, None)) => {
			// i-
			// [['i'], []]
			let _i = operands.0.unwrap();
			// unimplemented
			ins.set_opcode(0xe1);
			ins.set_disp(_i);
			Ok(ins)
		},
		("loopne", Operands(match_data!(Immediate(_, _, ..)), match_data!(Register(_, 1, 64, ..)), None, None)) => {
			// i-
			// [['i'], []]
			let _i = operands.0.unwrap();
			// unimplemented
			ins.set_opcode(0xe0);
			ins.set_disp(_i);
			Ok(ins)
		},
		("loopnz", Operands(match_data!(Immediate(_, _, ..)), match_data!(Register(_, 1, 64, ..)), None, None)) => {
			// i-
			// [['i'], []]
			let _i = operands.0.unwrap();
			// unimplemented
			ins.set_opcode(0xe0);
			ins.set_disp(_i);
			Ok(ins)
		},
		("loopz", Operands(match_data!(Immediate(_, _, ..)), match_data!(Register(_, 1, 64, ..)), None, None)) => {
			// i-
			// [['i'], []]
			let _i = operands.0.unwrap();
			// unimplemented
			ins.set_opcode(0xe1);
			ins.set_disp(_i);
			Ok(ins)
		},
		("lsl", Operands(match_data!(Register(_, _, 16, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_prefix(0x66);
			ins.set_rex_w();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x03);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lsl", Operands(match_data!(Register(_, _, 32, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			// 32 bit operand
			ins.set_rex_w();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x03);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lsl", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x03);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lsl", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 16, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x03);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lsl", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 32, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x03);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lsl", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x03);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("lss", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xb2);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("monitor", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Register(_, 1, 32, ..)), match_data!(Register(_, 2, 32, ..)), None)) => {
			// ---
			// [[], [], []]
			ins.set_opcode(0x0f);
			ins.set_opcode(0x01);
			ins.set_opcode(0xc8);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, _, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0x8c);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Register(_, _, _, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x8c);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, _, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0x8e);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, _, ..)), match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x8e);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// -i
			// [[], ['i']]
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xa1);
			ins.set_imm(_i);
			Ok(ins)
		},
		("mov", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, 0, 64, ..)), None, None)) => {
			// i-
			// [['i'], []]
			let _i = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xa3);
			ins.set_imm(_i);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, _, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x20);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, _, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x22);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, _, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x21);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, _, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x23);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("mov", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x89);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x89);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x8b);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x8b);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// ri
			// [['r'], ['i']]
			let _r = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex_w();
			ins.set_opecode_with_register(0xb8, _r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xc7);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			ins.set_imm(_i);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Immediate(_, _, ..)), None, None)) => {
			// ri
			// [['r'], ['i']]
			let _r = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opecode_with_register(0xb8, _r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, _, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0xc7);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			ins.set_imm(_i);
			Ok(ins)
		},
		("mov", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 32, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0xc7);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			ins.set_imm(_i);
			Ok(ins)
		},
		("movd", Operands(None, match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x6e);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("movd", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x7e);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("movq", Operands(None, match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x6e);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("movq", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x7e);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("movsq", Operands(None, None, None, None)) => {
			// void
			ins.set_rex();
			ins.set_opcode(0xa5);
			Ok(ins)
		},
		("movsx", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 8, ..)) | match_data!(Memory{size:8, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xbe);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("movsx", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 16, ..)) | match_data!(Memory{size:16, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xbf);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("movsxd", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 32, ..)) | match_data!(Memory{size:32, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x63);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("movsx", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 32, ..)) | match_data!(Memory{size:32, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x63);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("movzx", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 8, ..)) | match_data!(Memory{size:8, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xb6);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("movzx", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 16, ..)) | match_data!(Memory{size:16, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xb7);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("mul", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xf7);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("neg", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0xf7);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			Ok(ins)
		},
		("nop", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1f);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("not", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0xf7);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			Ok(ins)
		},
		("or", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x09);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("or", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x09);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("or", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0b);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("or", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0b);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("or", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			ins.set_imm(_i);
			Ok(ins)
		},
		("or", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			ins.set_imm(_i);
			Ok(ins)
		},
		("or", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(_, _, ..)), None, None)) => {
			// -i
			// [[], ['i']]
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0d);
			ins.set_imm(_i);
			Ok(ins)
		},
		("or", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			ins.set_imm(_i);
			Ok(ins)
		},
		("or", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, _, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x81);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			ins.set_imm(_i);
			Ok(ins)
		},
		("pop", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// r
			// [['r']]
			let _r = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opecode_with_register(0x58, _r);
			Ok(ins)
		},
		("pop", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0x8f);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("popfq", Operands(None, None, None, None)) => {
			// void
			// 32 bit operand
			ins.set_opcode(0x9d);
			Ok(ins)
		},
		("push", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// r
			// [['r']]
			let _r = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opecode_with_register(0x50, _r);
			Ok(ins)
		},
		("push", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0xff);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			Ok(ins)
		},
		("push", Operands(match_data!(Immediate(..)), None, None, None)) => {
			// i
			// [['i']]
			let _i = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0x6a);
			ins.set_imm(_i);
			Ok(ins)
		},
		("push", Operands(match_data!(Immediate(_, 64, ..)), None, None, None)) => {
			// i
			// [['i']]
			let _i = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0x68);
			ins.set_imm(_i);
			Ok(ins)
		},
		("push", Operands(match_data!(Immediate(..)), None, None, None)) => {
			// i
			// [['i']]
			let _i = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0x6a);
			ins.set_imm(_i);
			Ok(ins)
		},
		("push", Operands(match_data!(Immediate(_, 32, ..)), None, None, None)) => {
			// i
			// [['i']]
			let _i = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0x68);
			ins.set_imm(_i);
			Ok(ins)
		},
		("pushfq", Operands(None, None, None, None)) => {
			// void
			// 32 bit operand
			ins.set_opcode(0x9c);
			Ok(ins)
		},
		("rcl", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			Ok(ins)
		},
		("rcl", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Register(_, 1, 8, ..)), None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd3);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			Ok(ins)
		},
		("rcl", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xc1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			ins.set_imm(_i);
			Ok(ins)
		},
		("rcr", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			Ok(ins)
		},
		("rcr", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Register(_, 1, 8, ..)), None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd3);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			Ok(ins)
		},
		("rcr", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xc1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			ins.set_imm(_i);
			Ok(ins)
		},
		("rdtscp", Operands(None, None, None, None)) => {
			// void
			ins.set_opcode(0x0f);
			ins.set_opcode(0x01);
			ins.set_opcode(0xf9);
			Ok(ins)
		},
		("retq", Operands(None, None, None, None)) => {
			// void
			ins.set_rex_w();
			ins.set_opcode(0xc3);
			Ok(ins)
		},
		("retq", Operands(match_data!(Immediate(_, _, ..)), None, None, None)) => {
			// i
			// [['i']]
			let _i = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0xc2);
			ins.set_imm(_i);
			Ok(ins)
		},
		("retfq", Operands(None, None, None, None)) => {
			// void
			ins.set_rex();
			ins.set_opcode(0xcb);
			Ok(ins)
		},
		("retfq", Operands(match_data!(Immediate(_, _, ..)), None, None, None)) => {
			// i
			// [['i']]
			let _i = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xca);
			ins.set_imm(_i);
			Ok(ins)
		},
		("retnq", Operands(None, None, None, None)) => {
			// void
			ins.set_rex_w();
			ins.set_opcode(0xc3);
			Ok(ins)
		},
		("retnq", Operands(match_data!(Immediate(_, _, ..)), None, None, None)) => {
			// i
			// [['i']]
			let _i = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0xc2);
			ins.set_imm(_i);
			Ok(ins)
		},
		("rol", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("rol", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Register(_, 1, 8, ..)), None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd3);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("rol", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xc1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			ins.set_imm(_i);
			Ok(ins)
		},
		("ror", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			Ok(ins)
		},
		("ror", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Register(_, 1, 8, ..)), None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd3);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			Ok(ins)
		},
		("ror", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xc1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			ins.set_imm(_i);
			Ok(ins)
		},
		("sal", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("sal", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Register(_, 1, 8, ..)), None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd3);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("sal", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xc1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			ins.set_imm(_i);
			Ok(ins)
		},
		("sar", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			Ok(ins)
		},
		("sar", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Register(_, 1, 8, ..)), None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd3);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			Ok(ins)
		},
		("sar", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xc1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			ins.set_imm(_i);
			Ok(ins)
		},
		("sbb", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x19);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("sbb", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x19);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("sbb", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x1b);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("sbb", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x1b);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("sbb", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			ins.set_imm(_i);
			Ok(ins)
		},
		("sbb", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			ins.set_imm(_i);
			Ok(ins)
		},
		("sbb", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(_, _, ..)), None, None)) => {
			// -i
			// [[], ['i']]
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x1d);
			ins.set_imm(_i);
			Ok(ins)
		},
		("sbb", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			ins.set_imm(_i);
			Ok(ins)
		},
		("sbb", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, _, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x81);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			ins.set_imm(_i);
			Ok(ins)
		},
		("scasq", Operands(None, None, None, None)) => {
			// void
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0xaf);
			Ok(ins)
		},
		("shl", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("shl", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Register(_, 1, 8, ..)), None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd3);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("shl", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xc1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			ins.set_imm(_i);
			Ok(ins)
		},
		("shld", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), match_data!(Immediate(_, _, ..)), None)) => {
			// mri
			// [['m'], ['r'], ['i']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xa4);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("shld", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), match_data!(Immediate(_, _, ..)), None)) => {
			// mri
			// [['m'], ['r'], ['i']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xa4);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("shld", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), match_data!(Register(_, 1, 8, ..)), None)) => {
			// mr-
			// [['m'], ['r'], []]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xa5);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("shld", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), match_data!(Register(_, 1, 8, ..)), None)) => {
			// mr-
			// [['m'], ['r'], []]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xa5);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("shr", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			Ok(ins)
		},
		("shr", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Register(_, 1, 8, ..)), None, None)) => {
			// m-
			// [['m'], []]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0xd3);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			Ok(ins)
		},
		("shr", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xc1);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			ins.set_imm(_i);
			Ok(ins)
		},
		("shrd", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), match_data!(Immediate(_, _, ..)), None)) => {
			// mri
			// [['m'], ['r'], ['i']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xac);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("shrd", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), match_data!(Immediate(_, _, ..)), None)) => {
			// mri
			// [['m'], ['r'], ['i']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xac);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("shrd", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), match_data!(Register(_, 1, 8, ..)), None)) => {
			// mr-
			// [['m'], ['r'], []]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xad);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("shrd", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), match_data!(Register(_, 1, 8, ..)), None)) => {
			// mr-
			// [['m'], ['r'], []]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xad);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("sldt", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x00);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("sldt", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x00);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("skinit", Operands(None, None, None, None)) => {
			// void
			ins.set_opcode(0x0f);
			ins.set_opcode(0x01);
			ins.set_opcode(0xde);
			Ok(ins)
		},
		("smsw", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x01);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("stosq", Operands(None, None, None, None)) => {
			// void
			ins.set_rex();
			ins.set_opcode(0xab);
			Ok(ins)
		},
		("str", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x00);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			Ok(ins)
		},
		("sub", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x29);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("sub", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x29);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("sub", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x2b);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("sub", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x2b);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("sub", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			ins.set_imm(_i);
			Ok(ins)
		},
		("sub", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			ins.set_imm(_i);
			Ok(ins)
		},
		("sub", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(_, _, ..)), None, None)) => {
			// -i
			// [[], ['i']]
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x2d);
			ins.set_imm(_i);
			Ok(ins)
		},
		("sub", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			ins.set_imm(_i);
			Ok(ins)
		},
		("sub", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, _, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x81);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			ins.set_imm(_i);
			Ok(ins)
		},
		("swapgs", Operands(None, None, None, None)) => {
			// void
			ins.set_opcode(0x0f);
			ins.set_opcode(0x01);
			ins.set_opcode(0xf8);
			Ok(ins)
		},
		("test", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x85);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("test", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x85);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("test", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x85);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("test", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(_, _, ..)), None, None)) => {
			// -i
			// [[], ['i']]
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xa9);
			ins.set_imm(_i);
			Ok(ins)
		},
		("test", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, _, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xf7);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			ins.set_imm(_i);
			Ok(ins)
		},
		("xadd", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xc1);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("xadd", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0xc1);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("xchg", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// -r
			// [[], ['r']]
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opecode_with_register(0x90, _r);
			Ok(ins)
		},
		("xchg", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, 0, 64, ..)), None, None)) => {
			// r-
			// [['r'], []]
			let _r = operands.0.unwrap();
			ins.set_rex();
			ins.set_opecode_with_register(0x90, _r);
			Ok(ins)
		},
		("xchg", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x87);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("xchg", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x87);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("xchg", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x87);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("xor", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x31);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("xor", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x31);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("xor", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x33);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("xor", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x33);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("xor", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			ins.set_imm(_i);
			Ok(ins)
		},
		("xor", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			ins.set_imm(_i);
			Ok(ins)
		},
		("xor", Operands(match_data!(Register(_, 0, 64, ..)), match_data!(Immediate(_, _, ..)), None, None)) => {
			// -i
			// [[], ['i']]
			let _i = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x35);
			ins.set_imm(_i);
			Ok(ins)
		},
		("xor", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x83);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			ins.set_imm(_i);
			Ok(ins)
		},
		("xor", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, _, ..)), None, None)) => {
			// mi
			// [['m'], ['i']]
			let _m = operands.0.unwrap();
			let _i = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x81);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			ins.set_imm(_i);
			Ok(ins)
		},
		("cmovcc", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x40);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("cmovcc", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x40);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("jcc", Operands(match_data!(Immediate(_, _, ..)), None, None, None)) => {
			// i
			// [['i']]
			let _i = operands.0.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x80);
			ins.set_disp(_i);
			Ok(ins)
		},
		("cvtsi2ss", Operands(None, match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xf3);
			ins.set_opcode(0x0f);
			ins.set_opcode(0x2a);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("cvtss2si", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xf3);
			ins.set_opcode(0x0f);
			ins.set_opcode(0x2d);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("cvtss2si", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xf3);
			ins.set_opcode(0x0f);
			ins.set_opcode(0x2d);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("cvttss2si", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xf3);
			ins.set_opcode(0x0f);
			ins.set_opcode(0x2c);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("movmskps", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			// unimplemented
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x50);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("fxrstor64", Operands(match_data!(Memory{size:_, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			// unimplemented
			ins.set_opcode(0x0f);
			ins.set_opcode(0xae);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			Ok(ins)
		},
		("fxsave64", Operands(match_data!(Memory{size:_, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			// unimplemented
			ins.set_opcode(0x0f);
			ins.set_opcode(0xae);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("movnti", Operands(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex();
			// unimplemented
			ins.set_opcode(0x0f);
			ins.set_opcode(0xc3);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("movq", Operands(None, match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_opcode(0x66);
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x6e);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("movq", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_opcode(0x66);
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x7e);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("pextrw", Operands(match_data!(Register(_, _, 64, ..)), None, match_data!(Immediate(_, _, ..)), None)) => {
			// rmi
			// [['r'], ['m'], ['i']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_opcode(0x66);
			ins.set_opcode(0x0f);
			ins.set_opcode(0xc5);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("pinsrw", Operands(None, match_data!(Register(_, _, 64, ..)), match_data!(Immediate(_, _, ..)), None)) => {
			// rmi
			// [['r'], ['m'], ['i']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_opcode(0x66);
			ins.set_opcode(0x0f);
			ins.set_opcode(0xc4);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("cvtsd2si", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xf2);
			ins.set_opcode(0x0f);
			ins.set_opcode(0x2d);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("cvtsd2si", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xf2);
			ins.set_opcode(0x0f);
			ins.set_opcode(0x2d);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("cvtsi2sd", Operands(None, match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xf2);
			ins.set_opcode(0x0f);
			ins.set_opcode(0x2a);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("cvttsd2si", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xf2);
			ins.set_opcode(0x0f);
			ins.set_opcode(0x2c);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("cvttsd2si", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Memory{size:_, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			ins.set_opcode(0xf2);
			ins.set_opcode(0x0f);
			ins.set_opcode(0x2c);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("movmskpd", Operands(match_data!(Register(_, _, 64, ..)), None, None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_opcode(0x66);
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x50);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("vmread", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Register(_, _, 64, ..)), None, None)) => {
			// mr
			// [['m'], ['r']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			ins.set_rex_w();
			// unimplemented
			ins.set_opcode(0x0f);
			ins.set_opcode(0x78);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("vmwrite", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex_w();
			// unimplemented
			ins.set_opcode(0x0f);
			ins.set_opcode(0x79);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("extractps", Operands(match_data!(Register(_, _, 64, ..)), None, match_data!(Immediate(_, 8, ..)), None)) => {
			// mri
			// [['m'], ['r'], ['i']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x66);
			ins.set_opcode(0x0f);
			ins.set_opcode(0x3a);
			ins.set_opcode(0x17);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("pextrb", Operands(match_data!(Register(_, _, 64, ..)), None, match_data!(Immediate(_, 8, ..)), None)) => {
			// mri
			// [['m'], ['r'], ['i']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex_w();
			ins.set_opcode(0x66);
			ins.set_opcode(0x0f);
			ins.set_opcode(0x3a);
			ins.set_opcode(0x14);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("pextrq", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, match_data!(Immediate(_, 8, ..)), None)) => {
			// mri
			// [['m'], ['r'], ['i']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x66);
			ins.set_opcode(0x0f);
			ins.set_opcode(0x3a);
			ins.set_opcode(0x16);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("pextrw", Operands(match_data!(Register(_, _, 64, ..)), None, match_data!(Immediate(_, 8, ..)), None)) => {
			// mri
			// [['m'], ['r'], ['i']]
			let _m = operands.0.unwrap();
			let _r = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x66);
			ins.set_opcode(0x0f);
			ins.set_opcode(0x3a);
			ins.set_opcode(0x15);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("pinsrq", Operands(None, match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), match_data!(Immediate(_, 8, ..)), None)) => {
			// rmi
			// [['r'], ['m'], ['i']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			let _i = operands.2.unwrap();
			ins.set_rex();
			ins.set_opcode(0x66);
			ins.set_opcode(0x0f);
			ins.set_opcode(0x3a);
			ins.set_opcode(0x22);
			ins.set_rm(_m);
			ins.set_reg(_r);
			ins.set_imm(_i);
			Ok(ins)
		},
		("crc32", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 8, ..)) | match_data!(Memory{size:8, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			// unimplemented
			ins.set_opcode(0x0f);
			ins.set_opcode(0x38);
			ins.set_opcode(0xf0);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("crc32", Operands(match_data!(Register(_, _, 64, ..)), match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None)) => {
			// rm
			// [['r'], ['m']]
			let _r = operands.0.unwrap();
			let _m = operands.1.unwrap();
			ins.set_rex();
			// unimplemented
			ins.set_opcode(0x0f);
			ins.set_opcode(0x38);
			ins.set_opcode(0xf1);
			ins.set_rm(_m);
			ins.set_reg(_r);
			Ok(ins)
		},
		("hint_nop0", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x18);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("hint_nop1", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x18);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			Ok(ins)
		},
		("hint_nop2", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x18);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			Ok(ins)
		},
		("hint_nop3", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x18);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			Ok(ins)
		},
		("hint_nop4", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x18);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("hint_nop5", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x18);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			Ok(ins)
		},
		("hint_nop6", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x18);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			Ok(ins)
		},
		("hint_nop7", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x18);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			Ok(ins)
		},
		("hint_nop8", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x19);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("hint_nop9", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x19);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			Ok(ins)
		},
		("hint_nop10", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x19);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			Ok(ins)
		},
		("hint_nop11", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x19);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			Ok(ins)
		},
		("hint_nop12", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x19);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("hint_nop13", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x19);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			Ok(ins)
		},
		("hint_nop14", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x19);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			Ok(ins)
		},
		("hint_nop15", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x19);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			Ok(ins)
		},
		("hint_nop16", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1a);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("hint_nop17", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1a);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			Ok(ins)
		},
		("hint_nop18", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1a);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			Ok(ins)
		},
		("hint_nop19", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1a);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			Ok(ins)
		},
		("hint_nop20", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1a);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("hint_nop21", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1a);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			Ok(ins)
		},
		("hint_nop22", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1a);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			Ok(ins)
		},
		("hint_nop23", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1a);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			Ok(ins)
		},
		("hint_nop24", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1b);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("hint_nop25", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1b);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			Ok(ins)
		},
		("hint_nop26", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1b);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			Ok(ins)
		},
		("hint_nop27", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1b);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			Ok(ins)
		},
		("hint_nop28", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1b);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("hint_nop29", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1b);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			Ok(ins)
		},
		("hint_nop30", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1b);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			Ok(ins)
		},
		("hint_nop31", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1b);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			Ok(ins)
		},
		("hint_nop32", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1c);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("hint_nop33", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1c);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			Ok(ins)
		},
		("hint_nop34", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1c);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			Ok(ins)
		},
		("hint_nop35", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1c);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			Ok(ins)
		},
		("hint_nop36", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1c);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("hint_nop37", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1c);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			Ok(ins)
		},
		("hint_nop38", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1c);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			Ok(ins)
		},
		("hint_nop39", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1c);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			Ok(ins)
		},
		("hint_nop40", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1d);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("hint_nop41", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1d);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			Ok(ins)
		},
		("hint_nop42", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1d);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			Ok(ins)
		},
		("hint_nop43", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1d);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			Ok(ins)
		},
		("hint_nop44", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1d);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("hint_nop45", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1d);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			Ok(ins)
		},
		("hint_nop46", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1d);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			Ok(ins)
		},
		("hint_nop47", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1d);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			Ok(ins)
		},
		("hint_nop48", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1e);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("hint_nop49", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1e);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			Ok(ins)
		},
		("hint_nop50", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1e);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			Ok(ins)
		},
		("hint_nop51", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1e);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			Ok(ins)
		},
		("hint_nop52", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1e);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("hint_nop53", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1e);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			Ok(ins)
		},
		("hint_nop54", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1e);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			Ok(ins)
		},
		("hint_nop55", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1e);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			Ok(ins)
		},
		("hint_nop56", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1f);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(0);
			Ok(ins)
		},
		("hint_nop57", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1f);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(1);
			Ok(ins)
		},
		("hint_nop58", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1f);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(2);
			Ok(ins)
		},
		("hint_nop59", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1f);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(3);
			Ok(ins)
		},
		("hint_nop60", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1f);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(4);
			Ok(ins)
		},
		("hint_nop61", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1f);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(5);
			Ok(ins)
		},
		("hint_nop62", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1f);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(6);
			Ok(ins)
		},
		("hint_nop63", Operands(match_data!(Register(_, _, 64, ..)) | match_data!(Memory{size:64, ..}), None, None, None)) => {
			// m
			// [['m']]
			let _m = operands.0.unwrap();
			ins.set_rex();
			ins.set_opcode(0x0f);
			ins.set_opcode(0x1f);
			ins.set_rm(_m);
			ins.set_mod_rm_reg(7);
			Ok(ins)
		},
		_ => Err(operands),
	}
}


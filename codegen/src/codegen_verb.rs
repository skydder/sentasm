use data::{
	Keyword, Register, Immediate, Memory, Data, DataSet, Preposition, Result, Verb, Sentence, Label, emit_error_msg
};
use macros::{match_data, let_prep, make_operands};
use crate::{Operands, Instruction, nasm};

pub fn codegen_verb(sentence: Sentence) -> Result<String> {
	match sentence.verb {
		Verb("add") => gen_ins_add(sentence),
		Verb("substract") => gen_ins_substract(sentence),
		Verb("multiply") => gen_ins_multiply(sentence),
		Verb("divide") => gen_ins_divide(sentence),
		Verb("move") => gen_ins_move(sentence),
		Verb("jump") => gen_ins_jump(sentence),
		Verb("and") => gen_ins_and(sentence),
		Verb("or") => gen_ins_or(sentence),
		Verb("xor") => gen_ins_xor(sentence),
		Verb("not") => gen_ins_not(sentence),
		Verb("negate") => gen_ins_negate(sentence),
		Verb("shift-right") => gen_ins_shift_right(sentence),
		Verb("shift-left") => gen_ins_shift_left(sentence),
		Verb("call") => gen_ins_call(sentence),
		Verb("compare") => gen_ins_compare(sentence),
		Verb("load-effective-address") => gen_ins_load_effective_address(sentence),
		Verb("push") => gen_ins_push(sentence),
		Verb("pop") => gen_ins_pop(sentence),
		Verb("set-byte") => gen_ins_set_byte(sentence),
		Verb("extend-*ax-reg") => gen_ins_extend__ax_reg(sentence),
		Verb("return") => gen_ins_return(sentence),
		Verb("halt") => gen_ins_halt(sentence),
		Verb("leave") => gen_ins_leave(sentence),
		Verb("no-operation") => gen_ins_no_operation(sentence),
		Verb("systemcall") => gen_ins_systemcall(sentence),
		_ => todo!(),
	}
}
fn gen_ins_add(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_to, "to");
	let_prep!(_as, "as");
	
	match (&_obj, &_to, &_as) {
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)), None) => Ok(nasm("add", make_operands!(_to, _obj))),
		(match_data!(Register(_, _, ..)), match_data!(Memory{size:_, ..}), None) => Ok(nasm("add", make_operands!(_to, _obj))),
		(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, ..)), None) => Ok(nasm("add", make_operands!(_to, _obj))),
		(match_data!(Immediate(_, _, _)), match_data!(Memory{size:_, ..}), None) => Ok(nasm("add", make_operands!(_to, _obj))),
		(match_data!(Immediate(_, _, _)), match_data!(Register(_, _, ..)), None) => Ok(nasm("add", make_operands!(_to, _obj))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_substract(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_from, "from");
	let_prep!(_as, "as");
	
	match (&_obj, &_from, &_as) {
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), None) => Ok(nasm("sub", make_operands!(_from, _obj))),
		(match_data!(Immediate(_, _, _)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), None) => Ok(nasm("sub", make_operands!(_from, _obj))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_multiply(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_by, "by");
	let_prep!(_as, "as");
	
	match (&_obj, &_by, &_as) {
		(None, match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), None) => Ok(nasm("mul", make_operands!(_by))),
		(None, match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Keyword("signed"))) => Ok(nasm("imul", make_operands!(_by))),
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Keyword("signed"))) => Ok(nasm("imul", make_operands!(_obj, _by))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_divide(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_by, "by");
	let_prep!(_as, "as");
	
	match (&_obj, &_by, &_as) {
		(None, match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), None) => Ok(nasm("div", make_operands!(_by))),
		(None, match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Keyword("signed"))) => Ok(nasm("idiv", make_operands!(_by))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_move(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_to, "to");
	let_prep!(_as, "as");
	let_prep!(_with, "with");
	
	match (&_obj, &_to, &_as, &_with) {
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), None, None) => Ok(nasm("mov", make_operands!(_to, _obj))),
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}) | match_data!(Label(_)), match_data!(Register(_, _, ..)), None, None) => Ok(nasm("mov", make_operands!(_to, _obj))),
		(match_data!(Immediate(_, _, _)), match_data!(Register(_, _, ..)), None, None) => Ok(nasm("mov", make_operands!(_to, _obj))),
		(match_data!(Immediate(_, _, _)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), None, None) => Ok(nasm("mov", make_operands!(_to, _obj))),
		(match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Register(_, 16 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(nasm("movsx", make_operands!(_to, _obj))),
		(match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Register(_, 32 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(nasm("movsx", make_operands!(_to, _obj))),
		(match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Register(_, 64 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(nasm("movsx", make_operands!(_to, _obj))),
		(match_data!(Register(_, 16 | 0, ..)) | match_data!(Memory{size:16 | 0, ..}), match_data!(Register(_, 32 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(nasm("movsx", make_operands!(_to, _obj))),
		(match_data!(Register(_, 16 | 0, ..)) | match_data!(Memory{size:16 | 0, ..}), match_data!(Register(_, 64 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(nasm("movsx", make_operands!(_to, _obj))),
		(match_data!(Register(_, 16 | 0, ..)) | match_data!(Memory{size:16 | 0, ..}), match_data!(Register(_, 16 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(nasm("movsxd", make_operands!(_to, _obj))),
		(match_data!(Register(_, 32 | 0, ..)) | match_data!(Memory{size:32 | 0, ..}), match_data!(Register(_, 32 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(nasm("movsxd", make_operands!(_to, _obj))),
		(match_data!(Register(_, 32 | 0, ..)) | match_data!(Memory{size:32 | 0, ..}), match_data!(Register(_, 64 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(nasm("movsxd", make_operands!(_to, _obj))),
		(match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Register(_, 16 | 0, ..)), None, match_data!(Keyword("zero-extention"))) => Ok(nasm("movzx", make_operands!(_to, _obj))),
		(match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Register(_, 32 | 0, ..)), None, match_data!(Keyword("zero-extention"))) => Ok(nasm("movzx", make_operands!(_to, _obj))),
		(match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Register(_, 64 | 0, ..)), None, match_data!(Keyword("zero-extention"))) => Ok(nasm("movzx", make_operands!(_to, _obj))),
		(match_data!(Register(_, 16 | 0, ..)) | match_data!(Memory{size:16 | 0, ..}), match_data!(Register(_, 32 | 0, ..)), None, match_data!(Keyword("zero-extention"))) => Ok(nasm("movzx", make_operands!(_to, _obj))),
		(match_data!(Register(_, 16 | 0, ..)) | match_data!(Memory{size:16 | 0, ..}), match_data!(Register(_, 64 | 0, ..)), None, match_data!(Keyword("zero-extention"))) => Ok(nasm("movzx", make_operands!(_to, _obj))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_jump(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_to, "to");
	let_prep!(_if, "if");
	
	match (&_obj, &_to, &_if) {
		(None, match_data!(Label(_)), None) => Ok(nasm("jmp", make_operands!(_to))),
		(None, match_data!(Label(_)), match_data!(Keyword("=="))) => Ok(nasm("je", make_operands!(_to))),
		(None, match_data!(Label(_)), match_data!(Keyword("!="))) => Ok(nasm("jne", make_operands!(_to))),
		(None, match_data!(Label(_)), match_data!(Keyword(">"))) => Ok(nasm("jg", make_operands!(_to))),
		(None, match_data!(Label(_)), match_data!(Keyword(">="))) => Ok(nasm("jge", make_operands!(_to))),
		(None, match_data!(Label(_)), match_data!(Keyword("<"))) => Ok(nasm("jl", make_operands!(_to))),
		(None, match_data!(Label(_)), match_data!(Keyword("<="))) => Ok(nasm("jle", make_operands!(_to))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_and(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_with, "with");
	
	match (&_obj, &_with) {
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Register(_, _, ..))) => Ok(nasm("and", make_operands!(_obj, _with))),
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..})) => Ok(nasm("and", make_operands!(_obj, _with))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_or(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_with, "with");
	
	match (&_obj, &_with) {
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Register(_, _, ..))) => Ok(nasm("or", make_operands!(_obj, _with))),
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..})) => Ok(nasm("or", make_operands!(_obj, _with))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_xor(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_with, "with");
	
	match (&_obj, &_with) {
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Register(_, _, ..))) => Ok(nasm("xor", make_operands!(_obj, _with))),
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..})) => Ok(nasm("xor", make_operands!(_obj, _with))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_not(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}) => Ok(nasm("not", make_operands!(_obj))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_negate(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}) => Ok(nasm("neg", make_operands!(_obj))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_shift_right(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_by, "by");
	
	match (&_obj, &_by) {
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Immediate(_, _, _))) => Ok(nasm("shr", make_operands!(_obj, _by))),
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Register("cl", _, ..))) => Ok(nasm("shr", make_operands!(_obj, _by))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_shift_left(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_by, "by");
	
	match (&_obj, &_by) {
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Immediate(_, _, _))) => Ok(nasm("shl", make_operands!(_obj, _by))),
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Register("cl", _, ..))) => Ok(nasm("shl", make_operands!(_obj, _by))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_call(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		match_data!(Label(_)) => Ok(nasm("call", make_operands!(_obj))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_compare(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_with, "with");
	
	match (&_obj, &_with) {
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Register(_, _, ..))) => Ok(nasm("cmp", make_operands!(_with, _obj))),
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..})) => Ok(nasm("cmp", make_operands!(_with, _obj))),
		(match_data!(Immediate(_, _, _)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..})) => Ok(nasm("cmp", make_operands!(_with, _obj))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_return(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		None => Ok(nasm("ret", make_operands!())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_leave(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		None => Ok(nasm("leave", make_operands!())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_no_operation(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		None => Ok(nasm("nop", make_operands!())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_systemcall(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		None => Ok(nasm("syscall", make_operands!())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_halt(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		None => Ok(nasm("hlt", make_operands!())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_load_effective_address(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_to, "to");
	
	match (&_obj, &_to) {
		(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, ..))) => Ok(nasm("lea", make_operands!(_to, _obj))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_pop(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		match_data!(Register(_, 16 | 0, ..)) => Ok(nasm("pop", make_operands!(_obj))),
		match_data!(Register(_, 64 | 0, ..)) => Ok(nasm("pop", make_operands!(_obj))),
		match_data!(Register(_, 16 | 0, ..)) | match_data!(Memory{size:16 | 0, ..}) => Ok(nasm("pop", make_operands!(_obj))),
		match_data!(Register(_, 64 | 0, ..)) | match_data!(Memory{size:64 | 0, ..}) => Ok(nasm("pop", make_operands!(_obj))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_push(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		match_data!(Register(_, 16 | 0, ..)) => Ok(nasm("push", make_operands!(_obj))),
		match_data!(Register(_, 64 | 0, ..)) => Ok(nasm("push", make_operands!(_obj))),
		match_data!(Register(_, 16 | 0, ..)) | match_data!(Memory{size:16 | 0, ..}) => Ok(nasm("push", make_operands!(_obj))),
		match_data!(Register(_, 64 | 0, ..)) | match_data!(Memory{size:64 | 0, ..}) => Ok(nasm("push", make_operands!(_obj))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_set_byte(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_to, "to");
	let_prep!(_if, "if");
	
	match (&_obj, &_to, &_if) {
		(None, match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), None) => Ok(nasm("set", make_operands!(_to))),
		(None, match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Keyword("=="))) => Ok(nasm("sete", make_operands!(_to))),
		(None, match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Keyword("!="))) => Ok(nasm("setne", make_operands!(_to))),
		(None, match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Keyword(">"))) => Ok(nasm("setg", make_operands!(_to))),
		(None, match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Keyword(">="))) => Ok(nasm("setge", make_operands!(_to))),
		(None, match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Keyword("<"))) => Ok(nasm("setl", make_operands!(_to))),
		(None, match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Keyword("<="))) => Ok(nasm("setle", make_operands!(_to))),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_extend__ax_reg(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	let_prep!(_by, "by");
	
	match (&_obj, &_by) {
		(None, match_data!(Keyword("16bit"))) => Ok(nasm("cwd", make_operands!())),
		(None, match_data!(Keyword("32bit"))) => Ok(nasm("cdq", make_operands!())),
		(None, match_data!(Keyword("64bit"))) => Ok(nasm("cqo", make_operands!())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}


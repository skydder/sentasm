use data::{
	Keyword, Register, Immediate, Memory, Data, DataSet, Preposition, Result, Verb, Sentence, Label, emit_error_msg
};
use macros::{match_data, let_prep, gen_nasm};

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
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)), None) => Ok(gen_nasm!("add", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, _, ..)), match_data!(Memory{size:_, ..}), None) => Ok(gen_nasm!("add", _to.unwrap(), _obj.unwrap())),
		(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, ..)), None) => Ok(gen_nasm!("add", _to.unwrap(), _obj.unwrap())),
		(match_data!(Immediate(_, _, _)), match_data!(Memory{size:_, ..}), None) => Ok(gen_nasm!("add", _to.unwrap(), _obj.unwrap())),
		(match_data!(Immediate(_, _, _)), match_data!(Register(_, _, ..)), None) => Ok(gen_nasm!("add", _to.unwrap(), _obj.unwrap())),
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
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), None) => Ok(gen_nasm!("sub", _from.unwrap(), _obj.unwrap())),
		(match_data!(Immediate(_, _, _)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), None) => Ok(gen_nasm!("sub", _from.unwrap(), _obj.unwrap())),
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
		(None, match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), None) => Ok(gen_nasm!("mul", _by.unwrap())),
		(None, match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Keyword("signed"))) => Ok(gen_nasm!("imul", _by.unwrap())),
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Keyword("signed"))) => Ok(gen_nasm!("imul", _obj.unwrap(), _by.unwrap())),
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
		(None, match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), None) => Ok(gen_nasm!("div", _by.unwrap())),
		(None, match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Keyword("signed"))) => Ok(gen_nasm!("idiv", _by.unwrap())),
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
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), None, None) => Ok(gen_nasm!("mov", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}) | match_data!(Label(_)), match_data!(Register(_, _, ..)), None, None) => Ok(gen_nasm!("mov", _to.unwrap(), _obj.unwrap())),
		(match_data!(Immediate(_, _, _)), match_data!(Register(_, _, ..)), None, None) => Ok(gen_nasm!("mov", _to.unwrap(), _obj.unwrap())),
		(match_data!(Immediate(_, _, _)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), None, None) => Ok(gen_nasm!("mov", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Register(_, 16 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(gen_nasm!("movsx", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Register(_, 32 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(gen_nasm!("movsx", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Register(_, 64 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(gen_nasm!("movsx", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, 16 | 0, ..)) | match_data!(Memory{size:16 | 0, ..}), match_data!(Register(_, 32 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(gen_nasm!("movsx", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, 16 | 0, ..)) | match_data!(Memory{size:16 | 0, ..}), match_data!(Register(_, 64 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(gen_nasm!("movsx", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, 16 | 0, ..)) | match_data!(Memory{size:16 | 0, ..}), match_data!(Register(_, 16 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(gen_nasm!("movsxd", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, 32 | 0, ..)) | match_data!(Memory{size:32 | 0, ..}), match_data!(Register(_, 32 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(gen_nasm!("movsxd", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, 32 | 0, ..)) | match_data!(Memory{size:32 | 0, ..}), match_data!(Register(_, 64 | 0, ..)), None, match_data!(Keyword("sign-extention"))) => Ok(gen_nasm!("movsxd", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Register(_, 16 | 0, ..)), None, match_data!(Keyword("zero-extention"))) => Ok(gen_nasm!("movzx", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Register(_, 32 | 0, ..)), None, match_data!(Keyword("zero-extention"))) => Ok(gen_nasm!("movzx", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Register(_, 64 | 0, ..)), None, match_data!(Keyword("zero-extention"))) => Ok(gen_nasm!("movzx", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, 16 | 0, ..)) | match_data!(Memory{size:16 | 0, ..}), match_data!(Register(_, 32 | 0, ..)), None, match_data!(Keyword("zero-extention"))) => Ok(gen_nasm!("movzx", _to.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, 16 | 0, ..)) | match_data!(Memory{size:16 | 0, ..}), match_data!(Register(_, 64 | 0, ..)), None, match_data!(Keyword("zero-extention"))) => Ok(gen_nasm!("movzx", _to.unwrap(), _obj.unwrap())),
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
		(None, match_data!(Label(_)), None) => Ok(gen_nasm!("jmp", _to.unwrap())),
		(None, match_data!(Label(_)), match_data!(Keyword("=="))) => Ok(gen_nasm!("je", _to.unwrap())),
		(None, match_data!(Label(_)), match_data!(Keyword("!="))) => Ok(gen_nasm!("jne", _to.unwrap())),
		(None, match_data!(Label(_)), match_data!(Keyword(">"))) => Ok(gen_nasm!("jg", _to.unwrap())),
		(None, match_data!(Label(_)), match_data!(Keyword(">="))) => Ok(gen_nasm!("jge", _to.unwrap())),
		(None, match_data!(Label(_)), match_data!(Keyword("<"))) => Ok(gen_nasm!("jl", _to.unwrap())),
		(None, match_data!(Label(_)), match_data!(Keyword("<="))) => Ok(gen_nasm!("jle", _to.unwrap())),
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
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Register(_, _, ..))) => Ok(gen_nasm!("and", _obj.unwrap(), _with.unwrap())),
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..})) => Ok(gen_nasm!("and", _obj.unwrap(), _with.unwrap())),
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
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Register(_, _, ..))) => Ok(gen_nasm!("or", _obj.unwrap(), _with.unwrap())),
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..})) => Ok(gen_nasm!("or", _obj.unwrap(), _with.unwrap())),
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
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Register(_, _, ..))) => Ok(gen_nasm!("xor", _obj.unwrap(), _with.unwrap())),
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..})) => Ok(gen_nasm!("xor", _obj.unwrap(), _with.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_not(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}) => Ok(gen_nasm!("not", _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_negate(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}) => Ok(gen_nasm!("neg", _obj.unwrap())),
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
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Immediate(_, _, _))) => Ok(gen_nasm!("shr", _obj.unwrap(), _by.unwrap())),
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Register("cl", _, ..))) => Ok(gen_nasm!("shr", _obj.unwrap(), _by.unwrap())),
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
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Immediate(_, _, _))) => Ok(gen_nasm!("shl", _obj.unwrap(), _by.unwrap())),
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Register("cl", _, ..))) => Ok(gen_nasm!("shl", _obj.unwrap(), _by.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_call(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		match_data!(Label(_)) => Ok(gen_nasm!("call", _obj.unwrap())),
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
		(match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..}), match_data!(Register(_, _, ..))) => Ok(gen_nasm!("cmp", _with.unwrap(), _obj.unwrap())),
		(match_data!(Register(_, _, ..)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..})) => Ok(gen_nasm!("cmp", _with.unwrap(), _obj.unwrap())),
		(match_data!(Immediate(_, _, _)), match_data!(Register(_, _, ..)) | match_data!(Memory{size:_, ..})) => Ok(gen_nasm!("cmp", _with.unwrap(), _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_return(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		None => Ok(gen_nasm!("ret")),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_leave(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		None => Ok(gen_nasm!("leave")),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_no_operation(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		None => Ok(gen_nasm!("nop")),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_systemcall(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		None => Ok(gen_nasm!("syscall")),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_halt(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		None => Ok(gen_nasm!("hlt")),
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
		(match_data!(Memory{size:_, ..}), match_data!(Register(_, _, ..))) => Ok(gen_nasm!("lea", _to.unwrap(), _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_pop(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		match_data!(Register(_, 16 | 0, ..)) => Ok(gen_nasm!("pop", _obj.unwrap())),
		match_data!(Register(_, 64 | 0, ..)) => Ok(gen_nasm!("pop", _obj.unwrap())),
		match_data!(Register(_, 16 | 0, ..)) | match_data!(Memory{size:16 | 0, ..}) => Ok(gen_nasm!("pop", _obj.unwrap())),
		match_data!(Register(_, 64 | 0, ..)) | match_data!(Memory{size:64 | 0, ..}) => Ok(gen_nasm!("pop", _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
fn gen_ins_push(sentence: Sentence) -> Result<String> {
	let_prep!(_obj, "obj");
	
	match &_obj {
		match_data!(Register(_, 16 | 0, ..)) => Ok(gen_nasm!("push", _obj.unwrap())),
		match_data!(Register(_, 64 | 0, ..)) => Ok(gen_nasm!("push", _obj.unwrap())),
		match_data!(Register(_, 16 | 0, ..)) | match_data!(Memory{size:16 | 0, ..}) => Ok(gen_nasm!("push", _obj.unwrap())),
		match_data!(Register(_, 64 | 0, ..)) | match_data!(Memory{size:64 | 0, ..}) => Ok(gen_nasm!("push", _obj.unwrap())),
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
		(None, match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), None) => Ok(gen_nasm!("set", _to.unwrap())),
		(None, match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Keyword("=="))) => Ok(gen_nasm!("sete", _to.unwrap())),
		(None, match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Keyword("!="))) => Ok(gen_nasm!("setne", _to.unwrap())),
		(None, match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Keyword(">"))) => Ok(gen_nasm!("setg", _to.unwrap())),
		(None, match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Keyword(">="))) => Ok(gen_nasm!("setge", _to.unwrap())),
		(None, match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Keyword("<"))) => Ok(gen_nasm!("setl", _to.unwrap())),
		(None, match_data!(Register(_, 8 | 0, ..)) | match_data!(Memory{size:8 | 0, ..}), match_data!(Keyword("<="))) => Ok(gen_nasm!("setle", _to.unwrap())),
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
		(None, match_data!(Keyword("16bit"))) => Ok(gen_nasm!("cwd")),
		(None, match_data!(Keyword("32bit"))) => Ok(gen_nasm!("cdq")),
		(None, match_data!(Keyword("64bit"))) => Ok(gen_nasm!("cqo")),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}


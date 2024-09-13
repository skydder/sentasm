use super::super::{
	data::{Keyword, Register, Immediate, Memory}, Code, Data, DataSet, Loc, Preposition, PrepositionPhrases, Result, Verb, Sentence
};

macro_rules! CaseSome {
	($data:pat) => {Some(DataSet {data:$data, loc:_})};
}
macro_rules! emit_error_msg {
	($msg:expr, $loc:expr) => {
		eprintln!("{}", format!("{}{}", $msg, $loc))
};
}

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
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _to = sentence.preposition_phrases.get_object(Preposition("to")).map_or_else(|| None, |date| date.expect_object());
	let _as = sentence.preposition_phrases.get_object(Preposition("as")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_to, &_as) {
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))), None) => Ok(format!("add {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Memory(Memory{size:_, ..})), None) => Ok(format!("add {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register(_, _, ..))), None) => Ok(format!("add {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Immediate(Immediate(_, _))), CaseSome!(Data::Memory(Memory{size:_, ..})), None) => Ok(format!("add {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Immediate(Immediate(_, _))), CaseSome!(Data::Register(Register(_, _, ..))), None) => Ok(format!("add {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_substract(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _from = sentence.preposition_phrases.get_object(Preposition("from")).map_or_else(|| None, |date| date.expect_object());
	let _as = sentence.preposition_phrases.get_object(Preposition("as")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_from, &_as) {
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), None) => Ok(format!("sub {:?}, {:?}", _from.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Immediate(Immediate(_, _))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), None) => Ok(format!("sub {:?}, {:?}", _from.unwrap(), _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_multiply(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _by = sentence.preposition_phrases.get_object(Preposition("by")).map_or_else(|| None, |date| date.expect_object());
	let _as = sentence.preposition_phrases.get_object(Preposition("as")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_by, &_as) {
		(None, CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), None) => Ok(format!("mul {:?}", _by.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Keyword(Keyword("signed")))) => Ok(format!("imul {:?}", _by.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Keyword(Keyword("signed")))) => Ok(format!("imul {:?}, {:?}", _obj.unwrap(), _by.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_divide(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _by = sentence.preposition_phrases.get_object(Preposition("by")).map_or_else(|| None, |date| date.expect_object());
	let _as = sentence.preposition_phrases.get_object(Preposition("as")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_by, &_as) {
		(None, CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), None) => Ok(format!("div {:?}", _by.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Keyword(Keyword("signed")))) => Ok(format!("idiv {:?}", _by.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_move(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _to = sentence.preposition_phrases.get_object(Preposition("to")).map_or_else(|| None, |date| date.expect_object());
	let _as = sentence.preposition_phrases.get_object(Preposition("as")).map_or_else(|| None, |date| date.expect_object());
	let _with = sentence.preposition_phrases.get_object(Preposition("with")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_to, &_as, &_with) {
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), None, None) => Ok(format!("mov {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})) | CaseSome!(Data::Label(_)), CaseSome!(Data::Register(Register(_, _, ..))), None, None) => Ok(format!("mov {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Immediate(Immediate(_, _))), CaseSome!(Data::Register(Register(_, _, ..))), None, None) => Ok(format!("mov {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Immediate(Immediate(_, _))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), None, None) => Ok(format!("mov {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Register(Register(_, 16 | 0, ..))), None, CaseSome!(Data::Keyword(Keyword("sign-extention")))) => Ok(format!("movsx {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Register(Register(_, 32 | 0, ..))), None, CaseSome!(Data::Keyword(Keyword("sign-extention")))) => Ok(format!("movsx {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Register(Register(_, 64 | 0, ..))), None, CaseSome!(Data::Keyword(Keyword("sign-extention")))) => Ok(format!("movsx {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 16 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:16 | 0, ..})), CaseSome!(Data::Register(Register(_, 32 | 0, ..))), None, CaseSome!(Data::Keyword(Keyword("sign-extention")))) => Ok(format!("movsx {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 16 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:16 | 0, ..})), CaseSome!(Data::Register(Register(_, 64 | 0, ..))), None, CaseSome!(Data::Keyword(Keyword("sign-extention")))) => Ok(format!("movsx {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 16 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:16 | 0, ..})), CaseSome!(Data::Register(Register(_, 16 | 0, ..))), None, CaseSome!(Data::Keyword(Keyword("sign-extention")))) => Ok(format!("movsxd {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 32 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:32 | 0, ..})), CaseSome!(Data::Register(Register(_, 32 | 0, ..))), None, CaseSome!(Data::Keyword(Keyword("sign-extention")))) => Ok(format!("movsxd {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 32 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:32 | 0, ..})), CaseSome!(Data::Register(Register(_, 64 | 0, ..))), None, CaseSome!(Data::Keyword(Keyword("sign-extention")))) => Ok(format!("movsxd {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Register(Register(_, 16 | 0, ..))), None, CaseSome!(Data::Keyword(Keyword("zero-extention")))) => Ok(format!("movzx {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Register(Register(_, 32 | 0, ..))), None, CaseSome!(Data::Keyword(Keyword("zero-extention")))) => Ok(format!("movzx {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Register(Register(_, 64 | 0, ..))), None, CaseSome!(Data::Keyword(Keyword("zero-extention")))) => Ok(format!("movzx {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 16 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:16 | 0, ..})), CaseSome!(Data::Register(Register(_, 32 | 0, ..))), None, CaseSome!(Data::Keyword(Keyword("zero-extention")))) => Ok(format!("movzx {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 16 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:16 | 0, ..})), CaseSome!(Data::Register(Register(_, 64 | 0, ..))), None, CaseSome!(Data::Keyword(Keyword("zero-extention")))) => Ok(format!("movzx {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_jump(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _to = sentence.preposition_phrases.get_object(Preposition("to")).map_or_else(|| None, |date| date.expect_object());
	let _if = sentence.preposition_phrases.get_object(Preposition("if")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_to, &_if) {
		(None, CaseSome!(Data::Label(_)), None) => Ok(format!("jmp {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Label(_)), CaseSome!(Data::Keyword(Keyword("==")))) => Ok(format!("je {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Label(_)), CaseSome!(Data::Keyword(Keyword("!=")))) => Ok(format!("jne {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Label(_)), CaseSome!(Data::Keyword(Keyword(">")))) => Ok(format!("jg {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Label(_)), CaseSome!(Data::Keyword(Keyword(">=")))) => Ok(format!("jge {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Label(_)), CaseSome!(Data::Keyword(Keyword("<")))) => Ok(format!("jl {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Label(_)), CaseSome!(Data::Keyword(Keyword("<=")))) => Ok(format!("jle {:?}", _to.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_and(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _with = sentence.preposition_phrases.get_object(Preposition("with")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_with) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register(_, _, ..)))) => Ok(format!("and {:?}, {:?}", _obj.unwrap(), _with.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..}))) => Ok(format!("and {:?}, {:?}", _obj.unwrap(), _with.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_or(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _with = sentence.preposition_phrases.get_object(Preposition("with")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_with) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register(_, _, ..)))) => Ok(format!("or {:?}, {:?}", _obj.unwrap(), _with.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..}))) => Ok(format!("or {:?}, {:?}", _obj.unwrap(), _with.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_xor(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _with = sentence.preposition_phrases.get_object(Preposition("with")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_with) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register(_, _, ..)))) => Ok(format!("xor {:?}, {:?}", _obj.unwrap(), _with.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..}))) => Ok(format!("xor {:?}, {:?}", _obj.unwrap(), _with.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_not(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..}))) => Ok(format!("not {:?}", _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_negate(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..}))) => Ok(format!("neg {:?}", _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_shift_right(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _by = sentence.preposition_phrases.get_object(Preposition("by")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_by) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Immediate(Immediate(_, _)))) => Ok(format!("shr {:?}, {:?}", _obj.unwrap(), _by.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register("cl", _, ..)))) => Ok(format!("shr {:?}, {:?}", _obj.unwrap(), _by.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_shift_left(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _by = sentence.preposition_phrases.get_object(Preposition("by")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_by) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Immediate(Immediate(_, _)))) => Ok(format!("shl {:?}, {:?}", _obj.unwrap(), _by.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register("cl", _, ..)))) => Ok(format!("shl {:?}, {:?}", _obj.unwrap(), _by.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_call(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(CaseSome!(Data::Label(_))) => Ok(format!("call {:?}", _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_compare(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _with = sentence.preposition_phrases.get_object(Preposition("with")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_with) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register(_, _, ..)))) => Ok(format!("cmp {:?}, {:?}", _with.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..}))) => Ok(format!("cmp {:?}, {:?}", _with.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Immediate(Immediate(_, _))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..}))) => Ok(format!("cmp {:?}, {:?}", _with.unwrap(), _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_return(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(None) => Ok(format!("ret", )),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_leave(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(None) => Ok(format!("leave", )),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_no_operation(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(None) => Ok(format!("nop", )),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_systemcall(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(None) => Ok(format!("syscall", )),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_halt(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(None) => Ok(format!("hlt", )),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_load_effective_address(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _to = sentence.preposition_phrases.get_object(Preposition("to")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_to) {
		(CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register(_, _, ..)))) => Ok(format!("lea {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_pop(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(CaseSome!(Data::Register(Register(_, 16 | 0, ..)))) => Ok(format!("pop {:?}", _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 64 | 0, ..)))) => Ok(format!("pop {:?}", _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 16 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:16 | 0, ..}))) => Ok(format!("pop {:?}", _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 64 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:64 | 0, ..}))) => Ok(format!("pop {:?}", _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_push(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(CaseSome!(Data::Register(Register(_, 16 | 0, ..)))) => Ok(format!("push {:?}", _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 64 | 0, ..)))) => Ok(format!("push {:?}", _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 16 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:16 | 0, ..}))) => Ok(format!("push {:?}", _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 64 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:64 | 0, ..}))) => Ok(format!("push {:?}", _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_set_byte(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _to = sentence.preposition_phrases.get_object(Preposition("to")).map_or_else(|| None, |date| date.expect_object());
	let _if = sentence.preposition_phrases.get_object(Preposition("if")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_to, &_if) {
		(None, CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), None) => Ok(format!("set {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Keyword(Keyword("==")))) => Ok(format!("sete {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Keyword(Keyword("!=")))) => Ok(format!("setne {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Keyword(Keyword(">")))) => Ok(format!("setg {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Keyword(Keyword(">=")))) => Ok(format!("setge {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Keyword(Keyword("<")))) => Ok(format!("setl {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Keyword(Keyword("<=")))) => Ok(format!("setle {:?}", _to.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_extend__ax_reg(sentence: Sentence) -> Result<String> {
	let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _by = sentence.preposition_phrases.get_object(Preposition("by")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_by) {
		(None, CaseSome!(Data::Keyword(Keyword("16bit")))) => Ok(format!("cwd", )),
		(None, CaseSome!(Data::Keyword(Keyword("32bit")))) => Ok(format!("cdq", )),
		(None, CaseSome!(Data::Keyword(Keyword("64bit")))) => Ok(format!("cqo", )),
		_ => {
			emit_error_msg!("unmatched operand", sentence.verb_loc);
			Err(())
		},
	}
}


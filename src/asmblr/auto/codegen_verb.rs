use super::super::{
	data::{Keyword, Register, Immediate, Memory}, Code, Data, DataSet, Loc, Preposition, PrepositionPhrases, Result, Verb
};

macro_rules! CaseSome {
	($data:pat) => {Some(DataSet {data:$data, loc:_})};
}
macro_rules! emit_error_msg {
($msg:expr, $loc:expr) => {
eprintln!("{}", format!("{}{}", $msg, $loc))
};
}

pub fn codegen_verb(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	match verb {
		Verb("add") => gen_ins_add(verb, verb_loc, object, preposition_phrases),
		Verb("substract") => gen_ins_substract(verb, verb_loc, object, preposition_phrases),
		Verb("multiply") => gen_ins_multiply(verb, verb_loc, object, preposition_phrases),
		Verb("divide") => gen_ins_divide(verb, verb_loc, object, preposition_phrases),
		Verb("move") => gen_ins_move(verb, verb_loc, object, preposition_phrases),
		Verb("jump") => gen_ins_jump(verb, verb_loc, object, preposition_phrases),
		Verb("and") => gen_ins_and(verb, verb_loc, object, preposition_phrases),
		Verb("or") => gen_ins_or(verb, verb_loc, object, preposition_phrases),
		Verb("xor") => gen_ins_xor(verb, verb_loc, object, preposition_phrases),
		Verb("not") => gen_ins_not(verb, verb_loc, object, preposition_phrases),
		Verb("negate") => gen_ins_negate(verb, verb_loc, object, preposition_phrases),
		Verb("shift-right") => gen_ins_shift_right(verb, verb_loc, object, preposition_phrases),
		Verb("shift-left") => gen_ins_shift_left(verb, verb_loc, object, preposition_phrases),
		Verb("call") => gen_ins_call(verb, verb_loc, object, preposition_phrases),
		Verb("compare") => gen_ins_compare(verb, verb_loc, object, preposition_phrases),
		Verb("load-effective-address") => gen_ins_load_effective_address(verb, verb_loc, object, preposition_phrases),
		Verb("push") => gen_ins_push(verb, verb_loc, object, preposition_phrases),
		Verb("pop") => gen_ins_pop(verb, verb_loc, object, preposition_phrases),
		Verb("set-byte") => gen_ins_set_byte(verb, verb_loc, object, preposition_phrases),
		Verb("extend-*ax-reg") => gen_ins_extend__ax_reg(verb, verb_loc, object, preposition_phrases),
		Verb("return") => gen_ins_return(verb, verb_loc, object, preposition_phrases),
		Verb("halt") => gen_ins_halt(verb, verb_loc, object, preposition_phrases),
		Verb("leave") => gen_ins_leave(verb, verb_loc, object, preposition_phrases),
		Verb("no-operation") => gen_ins_no_operation(verb, verb_loc, object, preposition_phrases),
		Verb("systemcall") => gen_ins_systemcall(verb, verb_loc, object, preposition_phrases),
		_ => todo!(),
	}
}
 fn gen_ins_add(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _to = _preposition_phrases.get_object(Preposition("to")).map_or_else(|| None, |date| date.expect_object());
	let _as = _preposition_phrases.get_object(Preposition("as")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_to, &_as) {
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Memory(Memory{size:_, ..})), None) => Ok(format!("add {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Memory(Memory{size:_, ..})), None) => Ok(format!("add {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Immediate(Immediate(_))), CaseSome!(Data::Memory(Memory{size:_, ..})), None) => Ok(format!("add {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Immediate(Immediate(_))), CaseSome!(Data::Register(Register(_, _, ..))), None) => Ok(format!("add {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_substract(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _from = _preposition_phrases.get_object(Preposition("from")).map_or_else(|| None, |date| date.expect_object());
	let _as = _preposition_phrases.get_object(Preposition("as")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_from, &_as) {
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), None) => Ok(format!("sub {:?}, {:?}", _from.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Immediate(Immediate(_))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), None) => Ok(format!("sub {:?}, {:?}", _from.unwrap(), _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_multiply(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _by = _preposition_phrases.get_object(Preposition("by")).map_or_else(|| None, |date| date.expect_object());
	let _as = _preposition_phrases.get_object(Preposition("as")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_by, &_as) {
		(None, CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), None) => Ok(format!("mul {:?}", _by.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Keyword(Keyword("signed")))) => Ok(format!("imul {:?}", _by.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Keyword(Keyword("signed")))) => Ok(format!("imul {:?}, {:?}", _obj.unwrap(), _by.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_divide(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _by = _preposition_phrases.get_object(Preposition("by")).map_or_else(|| None, |date| date.expect_object());
	let _as = _preposition_phrases.get_object(Preposition("as")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_by, &_as) {
		(None, CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), None) => Ok(format!("div {:?}", _by.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Keyword(Keyword("signed")))) => Ok(format!("idiv {:?}", _by.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_move(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _to = _preposition_phrases.get_object(Preposition("to")).map_or_else(|| None, |date| date.expect_object());
	let _as = _preposition_phrases.get_object(Preposition("as")).map_or_else(|| None, |date| date.expect_object());
	let _with = _preposition_phrases.get_object(Preposition("with")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_to, &_as, &_with) {
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), None, None) => Ok(format!("mov {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})) | CaseSome!(Data::Label(_)), CaseSome!(Data::Register(Register(_, _, ..))), None, None) => Ok(format!("mov {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Immediate(Immediate(_))), CaseSome!(Data::Register(Register(_, _, ..))), None, None) => Ok(format!("mov {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Immediate(Immediate(_))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), None, None) => Ok(format!("mov {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
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
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_jump(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _to = _preposition_phrases.get_object(Preposition("to")).map_or_else(|| None, |date| date.expect_object());
	let _if = _preposition_phrases.get_object(Preposition("if")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_to, &_if) {
		(None, CaseSome!(Data::Label(_)), None) => Ok(format!("jmp {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Label(_)), CaseSome!(Data::Keyword(Keyword("==")))) => Ok(format!("je {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Label(_)), CaseSome!(Data::Keyword(Keyword("!=")))) => Ok(format!("jne {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Label(_)), CaseSome!(Data::Keyword(Keyword(">")))) => Ok(format!("jg {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Label(_)), CaseSome!(Data::Keyword(Keyword(">=")))) => Ok(format!("jge {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Label(_)), CaseSome!(Data::Keyword(Keyword("<")))) => Ok(format!("jl {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Label(_)), CaseSome!(Data::Keyword(Keyword("<=")))) => Ok(format!("jle {:?}", _to.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_and(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _with = _preposition_phrases.get_object(Preposition("with")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_with) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register(_, _, ..)))) => Ok(format!("and {:?}, {:?}", _obj.unwrap(), _with.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..}))) => Ok(format!("and {:?}, {:?}", _obj.unwrap(), _with.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_or(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _with = _preposition_phrases.get_object(Preposition("with")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_with) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register(_, _, ..)))) => Ok(format!("or {:?}, {:?}", _obj.unwrap(), _with.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..}))) => Ok(format!("or {:?}, {:?}", _obj.unwrap(), _with.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_xor(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _with = _preposition_phrases.get_object(Preposition("with")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_with) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register(_, _, ..)))) => Ok(format!("xor {:?}, {:?}", _obj.unwrap(), _with.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..}))) => Ok(format!("xor {:?}, {:?}", _obj.unwrap(), _with.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_not(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..}))) => Ok(format!("not {:?}", _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_negate(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..}))) => Ok(format!("neg {:?}", _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_shift_right(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _by = _preposition_phrases.get_object(Preposition("by")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_by) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Immediate(Immediate(_)))) => Ok(format!("shr {:?}, {:?}", _obj.unwrap(), _by.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register("cl", _, ..)))) => Ok(format!("shr {:?}, {:?}", _obj.unwrap(), _by.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_shift_left(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _by = _preposition_phrases.get_object(Preposition("by")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_by) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Immediate(Immediate(_)))) => Ok(format!("shl {:?}, {:?}", _obj.unwrap(), _by.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register("cl", _, ..)))) => Ok(format!("shl {:?}, {:?}", _obj.unwrap(), _by.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_call(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(CaseSome!(Data::Label(_))) => Ok(format!("call {:?}", _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_compare(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _with = _preposition_phrases.get_object(Preposition("with")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_with) {
		(CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register(_, _, ..)))) => Ok(format!("cmp {:?}, {:?}", _with.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, _, ..))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..}))) => Ok(format!("cmp {:?}, {:?}", _with.unwrap(), _obj.unwrap())),
		(CaseSome!(Data::Immediate(Immediate(_))), CaseSome!(Data::Register(Register(_, _, ..))) | CaseSome!(Data::Memory(Memory{size:_, ..}))) => Ok(format!("cmp {:?}, {:?}", _with.unwrap(), _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_return(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(None) => Ok(format!("ret", )),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_leave(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(None) => Ok(format!("leave", )),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_no_operation(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(None) => Ok(format!("nop", )),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_systemcall(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(None) => Ok(format!("syscall", )),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_halt(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(None) => Ok(format!("hlt", )),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_load_effective_address(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _to = _preposition_phrases.get_object(Preposition("to")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_to) {
		(CaseSome!(Data::Memory(Memory{size:_, ..})), CaseSome!(Data::Register(Register(_, _, ..)))) => Ok(format!("lea {:?}, {:?}", _to.unwrap(), _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_pop(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(CaseSome!(Data::Register(Register(_, 16 | 0, ..)))) => Ok(format!("pop {:?}", _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 64 | 0, ..)))) => Ok(format!("pop {:?}", _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 16 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:16 | 0, ..}))) => Ok(format!("pop {:?}", _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 64 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:64 | 0, ..}))) => Ok(format!("pop {:?}", _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_push(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj) {
		(CaseSome!(Data::Register(Register(_, 16 | 0, ..)))) => Ok(format!("push {:?}", _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 64 | 0, ..)))) => Ok(format!("push {:?}", _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 16 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:16 | 0, ..}))) => Ok(format!("push {:?}", _obj.unwrap())),
		(CaseSome!(Data::Register(Register(_, 64 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:64 | 0, ..}))) => Ok(format!("push {:?}", _obj.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_set_byte(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _to = _preposition_phrases.get_object(Preposition("to")).map_or_else(|| None, |date| date.expect_object());
	let _if = _preposition_phrases.get_object(Preposition("if")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_to, &_if) {
		(None, CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), None) => Ok(format!("set {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Keyword(Keyword("==")))) => Ok(format!("sete {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Keyword(Keyword("!=")))) => Ok(format!("setne {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Keyword(Keyword(">")))) => Ok(format!("setg {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Keyword(Keyword(">=")))) => Ok(format!("setge {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Keyword(Keyword("<")))) => Ok(format!("setl {:?}", _to.unwrap())),
		(None, CaseSome!(Data::Register(Register(_, 8 | 0, ..))) | CaseSome!(Data::Memory(Memory{size:8 | 0, ..})), CaseSome!(Data::Keyword(Keyword("<=")))) => Ok(format!("setle {:?}", _to.unwrap())),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}
 fn gen_ins_extend__ax_reg(_verb: Verb, _verb_loc: Loc, _object: Option<DataSet>, _preposition_phrases: &mut PrepositionPhrases) -> Result<String> {
	let _obj = _object.map_or_else(|| None, |date| date.expect_object());
	let _by = _preposition_phrases.get_object(Preposition("by")).map_or_else(|| None, |date| date.expect_object());
	
	match (&_obj, &_by) {
		(None, CaseSome!(Data::Keyword(Keyword("16bit")))) => Ok(format!("cwd", )),
		(None, CaseSome!(Data::Keyword(Keyword("32bit")))) => Ok(format!("cdq", )),
		(None, CaseSome!(Data::Keyword(Keyword("64bit")))) => Ok(format!("cqo", )),
		_ => {
			emit_error_msg!("unmatched operand", _verb_loc);
			Err(())
		},
	}
}


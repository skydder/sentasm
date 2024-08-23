use crate::emit_error_msg;
use super::{
    gen_gen::{gen_ins_add, gen_ins_move, gen_ins_multiply, gen_ins_substract, gen_ins_divide, gen_ins_jump, gen_ins_and,gen_ins_or,gen_ins_xor,gen_ins_not, gen_ins_negate, gen_ins_shift_right, gen_ins_shift_left, gen_ins_call,gen_ins_compare,gen_ins_return,
        gen_ins_leave,gen_ins_no_operation, gen_ins_systemcall, gen_ins_halt,gen_ins_load_effective_address, gen_ins_pop, gen_ins_push,gen_ins_set_byte, gen_ins_extend__ax_reg}, Code, Data, DataSet, Keyword, Loc, Preposition, PrepositionPhrases, Result, Verb
};

pub fn codegen(code: Code) -> Result<String> {
    match code {
        Code::NullStmt => Ok(format!("")),
        Code::LabelDef(l) => Ok(format!("{}:", l.0)),
        Code::Section(l) => Ok(format!("section {}", l.0)),
        Code::Sentence {
            verb,
            verb_loc,
            object,
            mut preposition_phrases,
        } => Ok(format!(
            "\t{}",
            codegen_sentence(verb, verb_loc, object, &mut preposition_phrases)?
        )),
    }
}

fn codegen_sentence(
    verb: Verb,
    verb_loc: Loc,
    object: Option<DataSet>,
    mut preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    match verb {
        Verb("add") => gen_ins_add(verb, verb_loc, object, &mut preposition_phrases),
        Verb("substract") => gen_ins_substract(verb, verb_loc, object, &mut preposition_phrases),
        Verb("multiply") => gen_ins_multiply(verb, verb_loc, object, &mut preposition_phrases),
        Verb("divide") => gen_ins_divide(verb, verb_loc, object, &mut preposition_phrases),
        Verb("move") => gen_ins_move(verb, verb_loc, object, &mut preposition_phrases),
        Verb("jump") => gen_ins_jump(verb, verb_loc, object, &mut preposition_phrases),
        Verb("and") => gen_ins_and(verb, verb_loc, object, &mut preposition_phrases),
        Verb("or") => gen_ins_or(verb, verb_loc, object, &mut preposition_phrases),
        Verb("xor") => gen_ins_xor(verb, verb_loc, object, &mut preposition_phrases),
        Verb("not") => gen_ins_not(verb, verb_loc, object, &mut preposition_phrases),
        Verb("negate") => gen_ins_negate(verb, verb_loc, object, &mut preposition_phrases),
        Verb("shift-right") => gen_ins_shift_right(verb, verb_loc, object, &mut preposition_phrases),
        Verb("shift-left") => gen_ins_shift_left(verb, verb_loc, object, &mut preposition_phrases),
        Verb("call") => gen_ins_call(verb, verb_loc, object, &mut preposition_phrases),
        Verb("compare") => gen_ins_compare(verb, verb_loc, object, &mut preposition_phrases),
        Verb("pop") => gen_ins_pop(verb, verb_loc, object, &mut preposition_phrases),
        Verb("push") => gen_ins_push(verb, verb_loc, object, &mut preposition_phrases),
        Verb("load-effective-address") => {
            gen_ins_load_effective_address(verb, verb_loc, object, &mut preposition_phrases)
        }
        Verb("set-byte") => gen_ins_set_byte(verb, verb_loc, object, preposition_phrases),
        Verb("extend-*ax-reg") => gen_ins_extend__ax_reg(verb, verb_loc, object, preposition_phrases),

        Verb("define") => gen_ins_def(verb, verb_loc, object, preposition_phrases),
        Verb("globalize") => gen_ins_global(verb, verb_loc, object, preposition_phrases),
        Verb("allocate") => gen_ins_alloc(verb, verb_loc, object, preposition_phrases),
        Verb("extern") => gen_ins_extern(verb, verb_loc, object, preposition_phrases),
        Verb("return") => gen_ins_return(verb, verb_loc, object, preposition_phrases),
        Verb("leave") => gen_ins_leave(verb, verb_loc, object, preposition_phrases),
        Verb("no-operation") => gen_ins_no_operation(verb, verb_loc, object, preposition_phrases),
        Verb("systemcall") => gen_ins_systemcall(verb, verb_loc, object, preposition_phrases),
        Verb("halt") => gen_ins_halt(verb, verb_loc, object, preposition_phrases),
        _ => {
            emit_error_msg!("Not implemented yet", verb_loc);
            Err(())
        }
    }
}

fn gen_ins_def(
    _verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let obj = object
        .map_or_else(|| None, |date| date.expect_label())
        .ok_or_else(|| eprintln!("expected label, but could not find it"))?;
    let az = preposition_phrases
        .get_object(Preposition("as"))
        .map_or_else(|| None, |date| date.expect_define())
        .ok_or_else(|| eprintln!("expected 'as' phrase, but could not find it"))?;
    let by = preposition_phrases
        .get_object(Preposition("by"))
        .map_or_else(|| None, |date| date.expect_keyword())
        .ok_or_else(|| eprintln!("expected 'by' phrase, but could not find it"))?;

    match (obj.data, az.data, by.data) {
        (Data::Label(l), Data::Define(i), Data::Keyword(super::data::Keyword("8bit"))) => {
            Ok(format!("{} db {:?}", l.0, i))
        }
        (Data::Label(l), Data::Define(i), Data::Keyword(super::data::Keyword("16bit"))) => {
            Ok(format!("{} dw {:?}", l.0, i))
        }
        (Data::Label(l), Data::Define(i), Data::Keyword(super::data::Keyword("32bit"))) => {
            Ok(format!("{} dd {:?}", l.0, i))
        }
        (Data::Label(l), Data::Define(i), Data::Keyword(super::data::Keyword("64bit"))) => {
            Ok(format!("{} dq {:?}", l.0, i))
        }
        _ => todo!(),
    }
}

fn gen_ins_global(
    _verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    _preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let obj = object
        .map_or_else(|| None, |date| date.expect_label())
        .ok_or_else(|| eprintln!("expected label, but could not find it"))?;
    Ok(format!("global {:?}", obj))
}

fn gen_ins_alloc(
    _verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let obj = object
        .map_or_else(|| None, |date| date.expect_label())
        .ok_or_else(|| eprintln!("expected label, but could not find it"))?;
    let vor: DataSet = preposition_phrases
        .get_object(Preposition("for"))
        .map_or_else(|| None, |date| date.expect_immediate())
        .ok_or_else(|| eprintln!("expected 'for' phrase, but could not find it"))?;
    let by = preposition_phrases
        .get_object(Preposition("by"))
        .map_or_else(|| None, |date| date.expect_keyword())
        .ok_or_else(|| eprintln!("expected 'by' phrase, but could not find it"))?;

    match (obj.data, vor.data, by.data) {
        (Data::Label(l), Data::Immediate(i), Data::Keyword(super::data::Keyword("8bit"))) => {
            Ok(format!("{} resb {}", l.0, i.0))
        }
        (Data::Label(l), Data::Immediate(i), Data::Keyword(super::data::Keyword("16bit"))) => {
            Ok(format!("{} resw {}", l.0, i.0))
        }
        (Data::Label(l), Data::Immediate(i), Data::Keyword(super::data::Keyword("32bit"))) => {
            Ok(format!("{} resd {}", l.0, i.0))
        }
        (Data::Label(l), Data::Immediate(i), Data::Keyword(super::data::Keyword("64bit"))) => {
            Ok(format!("{} resq {}", l.0, i.0))
        }
        _ => todo!(),
    }
}

fn gen_ins_extern(
    _verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    _preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let obj = object
        .map_or_else(|| None, |date| date.expect_label())
        .ok_or_else(|| eprintln!("expected label, but could not find it"))?;
    Ok(format!("extern {:?}", obj))
}

use super::{data::Verb, Code, DataSet, Result, Loc, PrepositionPhrases, Preposition};

pub fn codegen(code: Code) -> Result<String> {
    match code {
        Code::NullStmt=> Ok(format!("")),
        Code::LabelDef(l) => Ok(format!("{}:", l)),
        Code::Sentence { verb, verb_loc, object, preposition_phrases } => codegen_sentence(verb, verb_loc, object, preposition_phrases)
    }
}

fn codegen_sentence(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    match verb {
        Verb::Add => gen_ins_add(verb, verb_loc, object, preposition_phrases),
            Verb::Substract => gen_ins_sub(verb, verb_loc, object, preposition_phrases),
            Verb::Multiply => gen_ins_mul(verb, verb_loc, object, preposition_phrases),
            Verb::Divide => gen_ins_div(verb, verb_loc, object, preposition_phrases),
            Verb::Move => gen_ins_mov(verb, verb_loc, object, preposition_phrases),
            Verb::Jump => gen_ins_jmp(verb, verb_loc, object, preposition_phrases),
            Verb::And => gen_ins_and(verb, verb_loc, object, preposition_phrases),
            Verb::Or => gen_ins_or(verb, verb_loc, object, preposition_phrases),
            Verb::Xor => gen_ins_xor(verb, verb_loc, object, preposition_phrases),
            Verb::Not => gen_ins_not(verb, verb_loc, object, preposition_phrases),
            Verb::Negate => gen_ins_neg(verb, verb_loc, object, preposition_phrases),
            Verb::ShiftRight => gen_ins_shr(verb, verb_loc, object, preposition_phrases),
            Verb::ShiftLeft => gen_ins_shl(verb, verb_loc, object, preposition_phrases),
            Verb::Call => gen_ins_call(verb, verb_loc, object, preposition_phrases),
            Verb::Compare => gen_ins_cmp(verb, verb_loc, object, preposition_phrases),
            Verb::Return => gen_ins_ret(verb, verb_loc, object, preposition_phrases),
            Verb::Leave => gen_ins_leave(verb, verb_loc, object, preposition_phrases),
            Verb::NoOperation => gen_ins_nop(verb, verb_loc, object, preposition_phrases),
            Verb::SystemCall => gen_ins_syscall(verb, verb_loc, object, preposition_phrases),
            Verb::Halt => gen_ins_hlt(verb, verb_loc, object, preposition_phrases),
    }
}

fn gen_ins_add(verb: Verb, verb_loc: Loc, object: Option<DataSet>, mut preposition_phrases:PrepositionPhrases) -> Result<String> {
    let to = preposition_phrases.get_object(Preposition::To).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    
    Ok(format!("add {:?}, {:?}", to, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_sub(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_mul(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_div(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_mov(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_jmp(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_and(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_or(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_xor(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_not(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_neg(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_shr(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_shl(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_call(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_cmp(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_ret(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_leave(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_nop(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_syscall(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
fn gen_ins_hlt(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}

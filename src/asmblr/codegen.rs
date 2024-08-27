use super::{
    codegen_verb, Code, Data, DataSet, Loc, Preposition, PrepositionPhrases, Result, Verb
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
        Verb("define") => gen_ins_def(verb, verb_loc, object, preposition_phrases),
        Verb("globalize") => gen_ins_global(verb, verb_loc, object, preposition_phrases),
        Verb("allocate") => gen_ins_alloc(verb, verb_loc, object, preposition_phrases),
        Verb("extern") => gen_ins_extern(verb, verb_loc, object, preposition_phrases),
        _ => codegen_verb(verb, verb_loc, object, preposition_phrases)
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

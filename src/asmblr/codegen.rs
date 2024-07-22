use super::{data::Verb, Code, DataSet, Result, Loc, PrepositionPhrases};

pub fn codegen(code: Code) -> Result<String> {
    match code {
        Code::NullStmt=> Ok(format!("")),
        Code::LabelDef(l) => Ok(format!("{}:", l)),
        Code::Sentence { verb, verb_loc, object, preposition_phrases } => codegen_sentence(verb, verb_loc, object, preposition_phrases)
    }
}

fn codegen_sentence(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
    todo!();
}
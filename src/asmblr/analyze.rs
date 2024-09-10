use super::{
    codegen_verb, parser::Sentence, Code, Data, DataSet, Loc, Preposition, PrepositionPhrases, Register, Result, Verb
};
type Operands<'a> = (Option<DataSet<'a>>, Option<DataSet<'a>>, Option<DataSet<'a>>, Option<DataSet<'a>>);

macro_rules! CaseSome {
	($data:pat) => {Some(DataSet {data:$data, loc:_})};
}

fn codegen(ins: &str, operands: Operands) -> Result<String> {
    match ins {
        "add" => {
            match &operands {
                (CaseSome!(Data::Register(Register(_, 8, val0, _))), CaseSome!(Data::Register(Register(_, 8, val1, _))),  None, None) => {
                    todo!()
                },
                _ => todo!()
            }
        },
        _ => todo!()
    }
    
}

fn analyze_add(sentence: Sentence) -> Result<String> {
    let _obj = sentence.preposition_phrases.get_object(Preposition("obj")).map_or_else(|| None, |date| date.expect_object());
	let _to = sentence.preposition_phrases.get_object(Preposition("to")).map_or_else(|| None, |date| date.expect_object());
    return codegen("add", (_to, _obj, None, None))
}
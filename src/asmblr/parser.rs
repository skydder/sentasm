use super::{Label, DataSet, Loc, Verb};

pub(crate) enum Code<'a> {
    Sentence {
        verb: Verb,
        verb_loc: Loc<'a>,
        object: DataSet<'a>,
        // preposition_phrases: 
    },
    LabelDef(Label<'a>),
    NullStmt,
}
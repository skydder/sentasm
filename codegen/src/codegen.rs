use macros::get_prep_object;
use tokenizer::emit_error;

use crate::{
    codegen_verb, Sentence, Code, Data, Preposition, Result, Verb, Keyword
};

pub fn codegen(code: Code, asm: &mut String) -> Result<()> {
    let line = match code {
        Code::NullStmt => Ok(format!("")),
        Code::LabelDef(l) => Ok(format!("{}:\n", l.0)),
        Code::Section(l) => Ok(format!("section {}\n", l.0)),
        Code::Sentence(sentense) => Ok(format!(
            "\t{}\n",
            codegen_sentence(sentense)?
        )),
        Code::RawNasm(nasm) => Ok(format!("\t{}\n", nasm)),
    }?;
    asm.push_str(&line);
    Ok(())
}

fn codegen_sentence(
    sentence: Sentence
) -> Result<String> {
    match &sentence.verb {
        Verb("define") => gen_ins_def(sentence),
        Verb("globalize") => gen_ins_global(sentence),
        Verb("allocate") => gen_ins_alloc(sentence),
        Verb("extern") => gen_ins_extern(sentence),
        _ => codegen_verb(sentence)
    }
}

macro_rules! check_if {
    ($checkee: expr, $checker: expr, $loc: expr, $($msgs: expr), *) => {
        if let Some(data) = $checkee.map_or_else(|| None, $checker) {
            data
        } else {
            // error
            emit_error!($loc, $($msgs), *);
        }
    };
}
macro_rules! get_prep_object2 {
    ($sentence:expr, $prep: expr, $loc: expr, $checker: expr, $($msgs: expr), *) => {
        {
            let (obj, obj_loc) = get_prep_object!($sentence, $prep, $loc);
            check_if!(obj, $checker, obj_loc, $($msgs), *)
        }
    };
}

fn gen_ins_def(
    sentence: Sentence
) -> Result<String> {
    let obj = get_prep_object2!(sentence, "obj", sentence.location,  |date| date.expect_label(), "expected label, but could not find it");
    let _as = get_prep_object2!(sentence, "as", sentence.location,  |date| date.expect_define(), "expected 'as' phrase, but could not find it");
    let _by = get_prep_object2!(sentence, "by", sentence.location,  |date| date.expect_keyword(), "expected keyword, but could not find it");
    match (obj.data, _as.data, _by.data) {
        (Data::Label(l), Data::Define(i), Data::Keyword(Keyword("8bit"))) => {
            Ok(format!("{} db {}", l.0, i))
        }
        (Data::Label(l), Data::Define(i), Data::Keyword(Keyword("16bit"))) => {
            Ok(format!("{} dw {}", l.0, i))
        }
        (Data::Label(l), Data::Define(i), Data::Keyword(Keyword("32bit"))) => {
            Ok(format!("{} dd {}", l.0, i))
        }
        (Data::Label(l), Data::Define(i), Data::Keyword(Keyword("64bit"))) => {
            Ok(format!("{} dq {}", l.0, i))
        }
        _ => todo!(),
    }
}

fn gen_ins_global(
    sentence: Sentence
) -> Result<String> {
    let obj = get_prep_object2!(sentence, "obj", sentence.location,  |date| date.expect_label(), "expected label, but could not find it");

    Ok(format!("global {}", obj))
}

fn gen_ins_alloc(
    sentence: Sentence
) -> Result<String> {
    let obj = get_prep_object2!(sentence, "obj", sentence.location,  |date| date.expect_label(), "expected label, but could not find it");
    let _for = get_prep_object2!(sentence, "for", sentence.location,  |date| date.expect_immediate(), "expected immediate, but could not find it");
    let _by = get_prep_object2!(sentence, "by", sentence.location,  |date| date.expect_keyword(), "expected keyword, but could not find it");

    match (obj.data, _for.data, _by.data) {
        (Data::Label(l), Data::Immediate(i), Data::Keyword(Keyword("8bit"))) => {
            Ok(format!("{} resb {}", l.0, i.0))
        }
        (Data::Label(l), Data::Immediate(i), Data::Keyword(Keyword("16bit"))) => {
            Ok(format!("{} resw {}", l.0, i.0))
        }
        (Data::Label(l), Data::Immediate(i), Data::Keyword(Keyword("32bit"))) => {
            Ok(format!("{} resd {}", l.0, i.0))
        }
        (Data::Label(l), Data::Immediate(i), Data::Keyword(Keyword("64bit"))) => {
            Ok(format!("{} resq {}", l.0, i.0))
        }
        _ => todo!(),
    }
}

fn gen_ins_extern(
    sentence: Sentence
) -> Result<String> {
    let obj = get_prep_object2!(sentence, "obj", sentence.location,  |date| date.expect_label(), "expected label, but could not find it");
    Ok(format!("extern {}", obj))
}


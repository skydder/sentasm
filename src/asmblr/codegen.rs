use super::{
    codegen_verb, parser::Sentence, Code, Data, DataSet, Loc, Preposition, PrepositionPhrases, Result, Verb
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
    }?;
    asm.push_str(&line);
    Ok(())
}

fn codegen_sentence(
    // verb: Verb,
    // verb_loc: Loc,
    // object: Option<DataSet>,
    // mut preposition_phrases: &mut PrepositionPhrases,
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

fn gen_ins_def(
    sentence: Sentence
) -> Result<String> {
    let obj = sentence.preposition_phrases
        .get_object(Preposition("obj"))
        .map_or_else(|| None, |date| date.expect_label())
        .ok_or_else(|| eprintln!("expected label, but could not find it"))?;

    let az = sentence.preposition_phrases
        .get_object(Preposition("as"))
        .map_or_else(|| None, |date| date.expect_define())
        .ok_or_else(|| eprintln!("expected 'as' phrase, but could not find it"))?;
    let by = sentence.preposition_phrases
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
    sentence: Sentence
) -> Result<String> {
    let obj = sentence.preposition_phrases
        .get_object(Preposition("obj"))
        .map_or_else(|| None, |date| date.expect_label())
        .ok_or_else(|| eprintln!("expected label, but could not find it"))?;

    Ok(format!("global {:?}", obj))
}

fn gen_ins_alloc(
    sentence: Sentence
) -> Result<String> {
    let obj = sentence.preposition_phrases
        .get_object(Preposition("obj"))
        .map_or_else(|| None, |date| date.expect_label())
        .ok_or_else(|| eprintln!("expected label, but could not find it"))?;

    let vor: DataSet = sentence.preposition_phrases
        .get_object(Preposition("for"))
        .map_or_else(|| None, |date| date.expect_immediate())
        .ok_or_else(|| eprintln!("expected 'for' phrase, but could not find it"))?;
    let by = sentence.preposition_phrases
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
    sentence: Sentence
) -> Result<String> {
    let obj = sentence.preposition_phrases
        .get_object(Preposition("obj"))
        .map_or_else(|| None, |date| date.expect_label())
        .ok_or_else(|| eprintln!("expected label, but could not find it"))?;

    Ok(format!("extern {:?}", obj))
}

enum Rex {
    REX = 0x40,
    W = 0b1000,
    R = 0b0100,
    X = 0b0010,
    B = 0b0001,
}

macro_rules! rex_prefix {
    ($rex: expr) => {
        ($rex as u8)
    };
    ($rex1: expr, $rex2: expr) => {
        ($rex1 as u8) | ($rex2 as u8)
    };
    ($rex1: expr, $($rex2: tt)*) => {
        ($rex1 as u8) | rex_prefix!($($rex2)*)
    };
}

fn sib(scale: u8, index: u8, base: u8) -> u8 {
    sib_scale(scale) << 6 | index << 3 | base
}

fn sib_scale(scale: u8) -> u8 {
    match scale {
        1 => 0b00,
        2 => 0b01,
        4 => 0b10,
        8 => 0b11,
        _ => panic!("unexpected!!")
    }
}

fn mod_rm(mode: u8, reg: u8, rm: u8) -> u8 {
    mode << 6 | reg << 3 | rm
}

fn instruction(opcode: Vec<u8>, prefix: Vec<u8>, mod_rm: u8, disp: Vec<u8>, imm: Vec<u8>) -> Vec<u8> {
    todo!()
}

fn put_byte(byte: u8) -> String {
    format!("db {:#02x}\n", byte)
} 
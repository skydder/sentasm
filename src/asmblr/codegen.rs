use crate::emit_error_msg;
use super::{
    data::Keyword, Code, Data, DataSet, Loc, Preposition, PrepositionPhrases, Result, Verb,
    
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

macro_rules! check_operand {
    ($obj:expr, $prep:expr) => {
        if let Data::Memory(_) = $obj.data {
            ();
        } else if let Data::Memory(_) = $prep.data {
            ();
        } else if let Data::Label(_) = $obj.data {
            ();
        } else if let Data::Label(_) = $prep.data {
            ();
        } else if let Data::Immediate(_) = $obj.data {
            if $obj.size() >= $prep.size() {
                emit_error_msg!(
                    "mismatched operand size!! refer to the document",
                    $obj.loc
                );
                return Err(());
            }
        } else if let Data::Immediate(_) = $prep.data {
            if $obj.size() <= $prep.size() {
                emit_error_msg!(
                    "mismatched operand size!! refer to the document",
                    $obj.loc
                );
                return Err(());
            }
        } else if $obj.size() != $prep.size() {
            emit_error_msg!(
                "mismatched operand size!! refer to the document",
                $obj.loc
            );
            return Err(());
        }
    };
}

fn codegen_sentence(
    verb: Verb,
    verb_loc: Loc,
    object: Option<DataSet>,
    mut preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    match verb {
        Verb("add") => gen_ins_add(verb, verb_loc, object, &mut preposition_phrases),
        Verb("substract") => gen_ins_sub(verb, verb_loc, object, &mut preposition_phrases),
        Verb("multiply") => gen_ins_mul(verb, verb_loc, object, &mut preposition_phrases),
        Verb("divide") => gen_ins_div(verb, verb_loc, object, &mut preposition_phrases),
        Verb("move") => gen_ins_mov(verb, verb_loc, object, &mut preposition_phrases),
        Verb("jump") => gen_ins_jmp(verb, verb_loc, object, &mut preposition_phrases),
        Verb("and") => gen_ins_and(verb, verb_loc, object, &mut preposition_phrases),
        Verb("or") => gen_ins_or(verb, verb_loc, object, &mut preposition_phrases),
        Verb("xor") => gen_ins_xor(verb, verb_loc, object, &mut preposition_phrases),
        Verb("not") => gen_ins_not(verb, verb_loc, object, &mut preposition_phrases),
        Verb("negate") => gen_ins_neg(verb, verb_loc, object, &mut preposition_phrases),
        Verb("shift-right") => gen_ins_shr(verb, verb_loc, object, &mut preposition_phrases),
        Verb("shift-left") => gen_ins_shl(verb, verb_loc, object, &mut preposition_phrases),
        Verb("call") => gen_ins_call(verb, verb_loc, object, &mut preposition_phrases),
        Verb("compare") => gen_ins_cmp(verb, verb_loc, object, &mut preposition_phrases),
        Verb("pop") => gen_ins_pop(verb, verb_loc, object, &mut preposition_phrases),
        Verb("push") => gen_ins_push(verb, verb_loc, object, &mut preposition_phrases),
        Verb("load-effective-address") => {
            gen_ins_lea(verb, verb_loc, object, &mut preposition_phrases)
        }
        Verb("set-byte") => gen_ins_set(verb, verb_loc, object, preposition_phrases),
        Verb("extend-*ax-reg") => gen_ins_extend(verb, verb_loc, object, preposition_phrases),

        Verb("define") => gen_ins_def(verb, verb_loc, object, preposition_phrases),
        Verb("globalize") => gen_ins_global(verb, verb_loc, object, preposition_phrases),
        Verb("allocate") => gen_ins_alloc(verb, verb_loc, object, preposition_phrases),
        Verb("extern") => gen_ins_extern(verb, verb_loc, object, preposition_phrases),
        Verb("return") => gen_ins_ret(verb, verb_loc, object, preposition_phrases),
        Verb("leave") => gen_ins_leave(verb, verb_loc, object, preposition_phrases),
        Verb("no-operation") => gen_ins_nop(verb, verb_loc, object, preposition_phrases),
        Verb("systemcall") => gen_ins_syscall(verb, verb_loc, object, preposition_phrases),
        Verb("halt") => gen_ins_hlt(verb, verb_loc, object, preposition_phrases),
        _ => {
            emit_error_msg!("Not implemented yet", verb_loc);
            Err(())
        }
    }
}

// macro_rules! CaseSome {
//     ($data:pat) => {Some(DataSet {data:$data, loc:_})};
// }

fn gen_ins_add(
    _verb: Verb,
    _verb_loc: Loc,
    _object: Option<DataSet>,
    _preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let _to = _preposition_phrases
        .get_object(Preposition("to"))
        .map_or_else(|| None, |date| date.expect_object())
        .ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    let _as = if let Some(ap) = _preposition_phrases.get_object(Preposition("as")) {
        format!("{:?}", ap)
    } else {
        format!("")
    };
    let obj = _object.ok_or_else(|| eprintln!("this instructoin needs object"))?;
    check_operand!(obj, _to);
    Ok(format!("{:?}{} {:?}, {:?}", _verb, _as, _to, obj))

    // match (&obj, &_to, &_as) {
    //     (CaseSome!(Data::Register(Register{size: 8, ..})), CaseSome!(Data::Register(Register{size: 8, ..})), None) => Ok(format!("add {:?}, {:?}", _to, obj)),
    //     (CaseSome!(Data::Register(Register{size: 16, ..})), CaseSome!(Data::Register(Register{size: 16, ..})), None) => Ok(format!("add {:?}, {:?}", _to, obj)),
    //     (CaseSome!(Data::Register(Register{size: 32, ..})), CaseSome!(Data::Register(Register{size: 32, ..})), None) => Ok(format!("add {:?}, {:?}", _to, obj)),
    //     (CaseSome!(Data::Register(Register{size: 64, ..})), CaseSome!(Data::Register(Register{size: 64, ..})), None) => Ok(format!("add {:?}, {:?}", _to, obj)),

    //     (CaseSome!(Data::Memory(Memory{size:0,..})), CaseSome!(Data::Register(Register{size: 8, ..})), None) => Ok(format!("add {:?}, {:?}", _to, obj)),
    //     (CaseSome!(Data::Memory(Memory{size:0,..})), CaseSome!(Data::Register(Register{size: 16, ..})), None) => Ok(format!("add {:?}, {:?}", _to, obj)),
    //     (CaseSome!(Data::Memory(Memory{size:0,..})), CaseSome!(Data::Register(Register{size: 32, ..})), None) => Ok(format!("add {:?}, {:?}", _to, obj)),
    //     (CaseSome!(Data::Memory(Memory{size:0,..})), CaseSome!(Data::Register(Register{size: 64, ..})), None) => Ok(format!("add {:?}, {:?}", _to, obj)),

    //     (CaseSome!(Data::Memory(Memory{size:0,..})), CaseSome!(Data::Register(Register{size: 8, ..})), None) => Ok(format!("add {:?}, {:?}", _to, obj)),
    //     (CaseSome!(Data::Memory(Memory{size:0,..})), CaseSome!(Data::Register(Register{size: 16, ..})), None) => Ok(format!("add {:?}, {:?}", _to, obj)),
    //     (CaseSome!(Data::Memory(Memory{size:0,..})), CaseSome!(Data::Register(Register{size: 32, ..})), None) => Ok(format!("add {:?}, {:?}", _to, obj)),
    //     (CaseSome!(Data::Memory(Memory{size:0,..})), CaseSome!(Data::Register(Register{size: 64, ..})), None) => Ok(format!("add {:?}, {:?}", _to, obj)),

    //     (None, None, None) => todo!(),
    //     _ => {
    //         eprintln!("mismatched operand size!! refer to the document{}",  _verb_loc);
    //         Err(())
    //     }
    // }
}

fn gen_ins_sub(
    verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let from = preposition_phrases
        .get_object(Preposition("from"))
        .ok_or_else(|| eprintln!("expected 'from' phrase, but could not find it"))?;
    let az = if let Some(ap) = preposition_phrases.get_object(Preposition("as")) {
        format!("{:?}", ap)
    } else {
        format!("")
    };

    let obj = object
        .map_or_else(|| None, |date| date.expect_object())
        .ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    check_operand!(obj, from);
    Ok(format!("{:?}{} {:?}, {:?}", verb, az, from, obj))
}

fn gen_ins_mul(
    verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let by = preposition_phrases
        .get_object(Preposition("by"))
        .ok_or_else(|| eprintln!("expected 'by' phrase, but could not find it"))?;
    let obj = object.map_or_else(|| None, |date| date.expect_object());
    let with = match preposition_phrases.get_object(Preposition("with")) {
        Some(DataSet {
            data: Data::Keyword(Keyword("sign-extention")),
            loc: _,
        }) => "i",
        None => "",
        _ => {
            eprintln!(
                "mul instruction don't take 'with' except the time when with 'sign-extenction'"
            );
            return Err(());
        }
    };

    match obj {
        Some(obj) => {
            check_operand!(obj, by);
            Ok(format!("{}{:?} {:?}, {:?}", with, verb, obj, by))
        }
        None => {
            if by.is_register() {
                eprintln!("mismatched operand size!! refer to the document");
                return Err(());
            }
            Ok(format!("{}{:?} {:?}", with, verb, by))
        }
    }
}

fn gen_ins_div(
    verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let with = match preposition_phrases.get_object(Preposition("with")) {
        Some(DataSet {
            data: Data::Keyword(Keyword("sign-extention")),
            loc: _,
        }) => "i",
        None => "",
        _ => {
            eprintln!(
                "mul instruction don't take 'with' except the time when with 'sign-extenction'"
            );
            return Err(());
        }
    };
    Ok(format!(
        "{}{:?} {:?}",
        with,
        verb,
        object
            .map_or_else(|| None, |date| date.expect_register())
            .ok_or_else(|| eprintln!("expected object, but could not find it.\n->{}", _verb_loc))?
    ))
}

fn gen_ins_mov(
    _verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    // eprintln!("{:?} {:?} {:?} {}", _verb, object, preposition_phrases, _verb_loc);
    let to = preposition_phrases
        .get_object(Preposition("to"))
        .ok_or_else(|| {
            eprintln!(
                "expected 'to' phrase, but could not find it\n->{}",
                _verb_loc
            )
        })?;

    let az = preposition_phrases
        .get_object(Preposition("as"))
        .map_or_else(|| None, |date| date.expect_keyword());
    // seprintln!("{:?}", object);
    let obj = object
        .map_or_else(|| None, |date| date.expect_object())
        .ok_or_else(|| eprintln!("expected object, but could not find it{}", _verb_loc))?;

    // eprintln!("{:?}", obj);
    let with = preposition_phrases
        .get_object(Preposition("with"))
        .map_or_else(|| None, |date| date.expect_keyword());
    match (az, with) {
        (None, None) => {
            check_operand!(obj, to);
            Ok(format!("mov {:?}, {:?}", to, obj))
        }
        (
            Some(DataSet {
                data: Data::Keyword(Keyword("single-precision-float")),
                loc: _,
            }),
            None,
        ) => Ok(format!("movss {:?}, {:?}", to, obj)),
        (
            Some(DataSet {
                data: Data::Keyword(Keyword("double-precision-float")),
                loc: _,
            }),
            None,
        ) => Ok(format!("movsd {:?}, {:?}", to, obj)),

        (
            None,
            Some(DataSet {
                data: Data::Keyword(Keyword("zero-extention")),
                loc: _,
            }),
        ) => Ok(format!("movzx {:?}, {:?}", to, obj)),
        (
            None,
            Some(DataSet {
                data: Data::Keyword(Keyword("sign-extention")),
                loc: _,
            }),
        ) => {
            // eprintln!("ok");
            if obj.size() > 16 && to.size() > 16 {
                Ok(format!("movsxd {:?}, {:?}", to, obj))
            } else if obj.size() == 16 && to.size() == 16 {
                Ok(format!("movsxd {:?}, {:?}", to, obj))
            } else if obj.size() <= 16 && to.size() > 16 {
                Ok(format!("movsx {:?}, {:?}", to, obj))
            } else {
                eprintln!("unmatched operand.{}", _verb_loc);
                return Err(());
            }
        }
        _ => todo!(),
    }

    // check_operand!(obj, to);
    // Ok(format!("{:?}{} {:?}, {:?}", verb, az, to, obj))
}

fn gen_ins_jmp(
    _verb: Verb,
    _verb_loc: Loc,
    _object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    // this code is specifying 'label' as the object, but in machine code, memory address also can be the object.
    let to = preposition_phrases
        .get_object(Preposition("to"))
        .map_or_else(|| None, |date| date.expect_label())
        .ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    let az = if let Some(ap) = preposition_phrases.get_object(Preposition("if")) {
        format!("j{:?}", ap)
    } else {
        format!("jmp")
    };

    Ok(format!("{} {:?}", az, to))
}
fn gen_ins_and(
    verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let with = preposition_phrases
        .get_object(Preposition("with"))
        .ok_or_else(|| eprintln!("expected 'with' phrase, but could not find it"))?;
    let obj = object
        .map_or_else(|| None, |date| date.expect_object())
        .ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    check_operand!(obj, with);
    Ok(format!("{:?} {:?}, {:?}", verb, with, obj))
}
fn gen_ins_or(
    verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let with = preposition_phrases
        .get_object(Preposition("with"))
        .ok_or_else(|| eprintln!("expected 'with' phrase, but could not find it"))?;
    let obj = object
        .map_or_else(|| None, |date| date.expect_object())
        .ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    check_operand!(obj, with);
    Ok(format!("{:?} {:?}, {:?}", verb, with, obj))
}
fn gen_ins_xor(
    verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let with = preposition_phrases
        .get_object(Preposition("with"))
        .ok_or_else(|| eprintln!("expected 'with' phrase, but could not find it"))?;
    let obj = object
        .map_or_else(|| None, |date| date.expect_object())
        .ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    check_operand!(obj, with);
    Ok(format!("{:?} {:?}, {:?}", verb, with, obj))
}
fn gen_ins_not(
    verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    _preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    Ok(format!(
        "{:?} {:?}",
        verb,
        object
            .map_or_else(|| None, |date| date.expect_register())
            .ok_or_else(|| eprintln!("expected object, but could not find it"))?
    ))
}
fn gen_ins_neg(
    verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    _preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    Ok(format!(
        "{:?} {:?}",
        verb,
        object
            .map_or_else(|| None, |date| date.expect_register())
            .ok_or_else(|| eprintln!("expected object, but could not find it"))?
    ))
}
fn gen_ins_shr(
    verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let by = preposition_phrases
        .get_object(Preposition("by"))
        .ok_or_else(|| eprintln!("expected 'by' phrase, but could not find it"))?;
    let obj = object
        .map_or_else(|| None, |date| date.expect_object())
        .ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    check_operand!(obj, by);
    Ok(format!("{:?} {:?}, {:?}", verb, obj, by))
}
fn gen_ins_shl(
    verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let by = preposition_phrases
        .get_object(Preposition("by"))
        .ok_or_else(|| eprintln!("expected 'by' phrase, but could not find it"))?;
    let obj = object
        .map_or_else(|| None, |date| date.expect_object())
        .ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    check_operand!(obj, by);
    Ok(format!("{:?} {:?}, {:?}", verb, obj, by))
}

fn gen_ins_call(
    verb: Verb,
    _verb_loc: Loc,
    _object: Option<DataSet>,
    _preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    // this code is specifying 'label' as the object, but in machine code, memory address also can be the object.
    let obj = _object
        .map_or_else(|| None, |date| date.expect_label())
        .ok_or_else(|| eprintln!("expected label, but could not find it"))?;
    Ok(format!("{:?} {:?}", verb, obj))
}
fn gen_ins_cmp(
    verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let to = preposition_phrases
        .get_object(Preposition("to"))
        .ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    let obj = object
        .map_or_else(|| None, |date| date.expect_object())
        .ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    check_operand!(obj, to);
    Ok(format!("{:?} {:?}, {:?}", verb, obj, to))
}
fn gen_ins_ret(
    _verb: Verb,
    _verb_loc: Loc,
    _object: Option<DataSet>,
    _preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    Ok(format!("ret"))
}
fn gen_ins_leave(
    _verb: Verb,
    _verb_loc: Loc,
    _object: Option<DataSet>,
    _preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    Ok(format!("leave"))
}
fn gen_ins_nop(
    _verb: Verb,
    _verb_loc: Loc,
    _object: Option<DataSet>,
    _preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    Ok(format!("nop"))
}
fn gen_ins_syscall(
    _verb: Verb,
    _verb_loc: Loc,
    _object: Option<DataSet>,
    _preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    Ok(format!("syscall"))
}
fn gen_ins_hlt(
    _verb: Verb,
    _verb_loc: Loc,
    _object: Option<DataSet>,
    _preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    // assert!(check_intransitive(object, preposition_phrases)); // todo: assert to error handling
    Ok(format!("hlt"))
}

fn gen_ins_lea(
    verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let to = preposition_phrases
        .get_object(Preposition("to"))
        .map_or_else(|| None, |date| date.expect_register())
        .ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    let obj = object
        .map_or_else(|| None, |date| date.expect_memory())
        .ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    // check_operand!(obj, to);
    Ok(format!("{:?} {:?}, {:?}", verb, to, obj))
}

fn gen_ins_pop(
    verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    _preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let obj = object
        .map_or_else(|| None, |date| date.expect_object())
        .ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    Ok(format!("{:?} {:?}", verb, obj))
}

fn gen_ins_push(
    verb: Verb,
    _verb_loc: Loc,
    object: Option<DataSet>,
    _preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    let obj = object
        .map_or_else(|| None, |date| date.expect_object())
        .ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    Ok(format!("{:?} {:?}", verb, obj))
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

fn gen_ins_set(
    _verb: Verb,
    _verb_loc: Loc,
    _object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    // this code is specifying 'label' as the object, but in machine code, memory address also can be the object.
    // eprintln!("{:?} {:?} {:?} {}", _verb, _object, preposition_phrases, _verb_loc);
    let to = preposition_phrases
        .get_object(Preposition("to"))
        .map_or_else(|| None, |date| date.expect_register())
        .ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    let az = if let Some(ap) = preposition_phrases.get_object(Preposition("if")) {
        match ap.data {
            Data::Keyword(Keyword("==")) => format!("sete"),
            Data::Keyword(Keyword("!=")) => format!("setne"),
            Data::Keyword(Keyword("<=")) => format!("setle"),
            Data::Keyword(Keyword(">=")) => format!("setge"),
            Data::Keyword(Keyword("<")) => format!("setl"),
            Data::Keyword(Keyword(">")) => format!("setg"),
            _ => {
                eprintln!("what{}", _verb_loc);
                return Err(());
            }
        }
        // format!("set{:?}", ap)
    } else {
        eprintln!("This instruction needs 'if'.{}", _verb_loc);
        return Err(());
    };

    Ok(format!("{} {:?}", az, to))
}

fn gen_ins_extend(
    _verb: Verb,
    _verb_loc: Loc,
    _object: Option<DataSet>,
    preposition_phrases: &mut PrepositionPhrases,
) -> Result<String> {
    // this code is specifying 'label' as the object, but in machine code, memory address also can be the object.
    match preposition_phrases.get_object(Preposition("by")) {
        Some(DataSet {
            data: Data::Keyword(Keyword("16bit")),
            loc: _,
        }) => Ok(format!("cwd")),
        Some(DataSet {
            data: Data::Keyword(Keyword("32bit")),
            loc: _,
        }) => Ok(format!("cdq")),
        Some(DataSet {
            data: Data::Keyword(Keyword("64bit")),
            loc: _,
        }) => Ok(format!("cqo")),
        _ => {
            eprintln!("unexpected!{}", _verb_loc);
            return Err(());
        }
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

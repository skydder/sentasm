use super::{data::{self, Verb}, Code, Data, DataSet, Keyword, Loc, Preposition, PrepositionPhrases, Result};

pub fn codegen(mut code: Code) -> Result<String> {
    match code {
        Code::NullStmt=> Ok(format!("")),
        Code::LabelDef(l) => Ok(format!("{}:", l)),
        Code::Sentence { verb, verb_loc, object,mut preposition_phrases } => codegen_sentence(verb, verb_loc, object,  &mut preposition_phrases)
    }
}

fn check_intransitive(object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> bool {
    if object.is_none() & preposition_phrases.expect_empty().ok().is_some() {
        true
    } else {
        false
    }
}

macro_rules! check_operand {
    ($obj:expr, $prep:expr) => {
        if let  Data::Immediate(_) = $obj.data {
            if $obj.size() >= $prep.size() {
                eprintln!("mismatched operand size!! refer to the document");
                return Err(());
            }
        } else if $obj.size() != $prep.size() & !$obj.size() & !$prep.size() {
            eprintln!("mismatched operand size!! refer to the document");
            return Err(());
        }
    };
}

fn codegen_sentence(verb: Verb, verb_loc: Loc, object: Option<DataSet>, mut preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    match verb {
        Verb::Add => gen_ins_add(verb, verb_loc, object, &mut preposition_phrases),
            Verb::Substract => gen_ins_sub(verb, verb_loc, object, &mut preposition_phrases),
            Verb::Multiply => gen_ins_mul(verb, verb_loc, object, &mut preposition_phrases),
            Verb::Divide => gen_ins_div(verb, verb_loc, object, &mut preposition_phrases),
            Verb::Move => gen_ins_mov(verb, verb_loc, object, &mut preposition_phrases),
            Verb::Jump => gen_ins_jmp(verb, verb_loc, object, &mut preposition_phrases),
            Verb::And => gen_ins_and(verb, verb_loc, object, &mut preposition_phrases),
            Verb::Or => gen_ins_or(verb, verb_loc, object, &mut preposition_phrases),
            Verb::Xor => gen_ins_xor(verb, verb_loc, object, &mut preposition_phrases),
            Verb::Not => gen_ins_not(verb, verb_loc, object, &mut preposition_phrases),
            Verb::Negate => gen_ins_neg(verb, verb_loc, object, &mut preposition_phrases),
            Verb::ShiftRight => gen_ins_shr(verb, verb_loc, object, &mut preposition_phrases),
            Verb::ShiftLeft => gen_ins_shl(verb, verb_loc, object, &mut preposition_phrases),
            Verb::Call => gen_ins_call(verb, verb_loc, object, &mut preposition_phrases),
            Verb::Compare => gen_ins_cmp(verb, verb_loc, object, &mut preposition_phrases),
            // Verb::Return => gen_ins_ret(verb, verb_loc, object, preposition_phrases),
            // Verb::Leave => gen_ins_leave(verb, verb_loc, object, preposition_phrases),
            // Verb::NoOperation => gen_ins_nop(verb, verb_loc, object, preposition_phrases),
            // Verb::SystemCall => gen_ins_syscall(verb, verb_loc, object, preposition_phrases),
            // Verb::Halt => gen_ins_hlt(verb, verb_loc, object, preposition_phrases),
            _ => gen_ins_iv(verb, verb_loc, object, &mut preposition_phrases),
    }
}

fn gen_ins_add(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let to = preposition_phrases.get_object(Preposition::To).map_or_else(|| None, |date| date.expect_object()).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    let az = if let Some(ap) = preposition_phrases.get_object(Preposition::As) {
        format!("{:?}", ap)
    } else {
        format!("")
    };
    let obj = object.map_or_else(|| None, |date| date.expect_object()).ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    if let  Data::Immediate(_) = obj.data {
        if obj.size() >= to.size() {
            eprintln!("mismatched operand size!! refer to the document");
            return Err(());
        }
    } else if obj.size() != to.size() & !obj.size() & !to.size() {
        eprintln!("mismatched operand size!! refer to the document");
        return Err(());
    }
    Ok(format!("{:?}{} {:?}, {:?}", verb, az, to, obj))
}

fn gen_ins_sub(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let from = preposition_phrases.get_object(Preposition::From).ok_or_else(|| eprintln!("expected 'from' phrase, but could not find it"))?;
    let az = if let Some(ap) = preposition_phrases.get_object(Preposition::As) {
        format!("{:?}", ap)
    } else {
        format!("")
    };

    let obj = object.map_or_else(|| None, |date| date.expect_object()).ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    if let  Data::Immediate(_) = obj.data {
        if obj.size() >= from.size() {
            eprintln!("mismatched operand size!! refer to the document");
            return Err(());
        }
    } else if obj.size() != from.size() & !obj.size() & !from.size() {
        eprintln!("mismatched operand size!! refer to the document");
        return Err(());
    }
    Ok(format!("{:?}{} {:?}, {:?}", verb, az, from, obj))
}

fn gen_ins_mul(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let by = preposition_phrases.get_object(Preposition::By).ok_or_else(|| eprintln!("expected 'by' phrase, but could not find it"))?;
    match object.map_or_else(|| None, |date| date.expect_object()) {
        Some(obj) => {
            if let  Data::Immediate(_) = obj.data {
                if obj.size() >= by.size() {
                    eprintln!("mismatched operand size!! refer to the document");
                    return Err(());
                }
            } else if obj.size() != by.size() & !obj.size() & !by.size() {
                eprintln!("mismatched operand size!! refer to the document");
                return Err(());
            }
            Ok(format!("{:?} {:?}, {:?}", verb, by, obj))
        },
        None => {
            if by.is_register() {
                eprintln!("mismatched operand size!! refer to the document");
                return Err(());
            }
            Ok(format!("{:?} {:?}", verb, by))
        }
    }
}

fn gen_ins_div(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    Ok(format!("{:?} {:?}", verb, object.map_or_else(|| None, |date| date.expect_register()).ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}

fn gen_ins_mov(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let to = preposition_phrases.get_object(Preposition::To).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    let az = if let Some(ap) = preposition_phrases.get_object(Preposition::As) {
        format!("{:?}", ap)
    } else {
        format!("")
    };
    let obj = object.map_or_else(|| None, |date| date.expect_object()).ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    if let  Data::Immediate(_) = obj.data {
        if obj.size() >= to.size() {
            eprintln!("mismatched operand size!! refer to the document");
            return Err(());
        }
    } else if obj.size() != to.size() & !obj.size() & !to.size() {
        eprintln!("mismatched operand size!! refer to the document");
        return Err(());
    }
    Ok(format!("{:?}{} {:?}, {:?}", verb, az, to, obj))
}

fn gen_ins_jmp(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    // this code is specifying 'label' as the object, but in machine code, memory address also can be the object.
    let to = preposition_phrases.get_object(Preposition::To).map_or_else(|| None, |date| date.expect_label()).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    let az = if let Some(ap) = preposition_phrases.get_object(Preposition::If) {
        format!("j{:?}", ap)
    } else {
        format!("jmp")
    };

    Ok(format!("{} {:?}", az, to))
}
fn gen_ins_and(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let with = preposition_phrases.get_object(Preposition::With).ok_or_else(|| eprintln!("expected 'with' phrase, but could not find it"))?;
    let obj = object.map_or_else(|| None, |date| date.expect_object()).ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    check_operand!(obj, with);
    Ok(format!("{:?} {:?}, {:?}", verb, with, obj))
}
fn gen_ins_or(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let with = preposition_phrases.get_object(Preposition::With).ok_or_else(|| eprintln!("expected 'with' phrase, but could not find it"))?;
    let obj = object.map_or_else(|| None, |date| date.expect_object()).ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    check_operand!(obj, with);
    Ok(format!("{:?} {:?}, {:?}", verb, with, obj))
}
fn gen_ins_xor(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let with = preposition_phrases.get_object(Preposition::With).ok_or_else(|| eprintln!("expected 'with' phrase, but could not find it"))?;
    let obj = object.map_or_else(|| None, |date| date.expect_object()).ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    check_operand!(obj, with);
    Ok(format!("{:?} {:?}, {:?}", verb, with, obj))
}
fn gen_ins_not(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    Ok(format!("{:?} {:?}", verb, object.map_or_else(|| None, |date| date.expect_register()).ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_neg(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    Ok(format!("{:?} {:?}", verb, object.map_or_else(|| None, |date| date.expect_register()).ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_shr(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let by = preposition_phrases.get_object(Preposition::By).ok_or_else(|| eprintln!("expected 'by' phrase, but could not find it"))?;
    let obj = object.map_or_else(|| None, |date| date.expect_object()).ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    check_operand!(obj, by);
    Ok(format!("{:?} {:?}, {:?}", verb, obj, by))
}
fn gen_ins_shl(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let by = preposition_phrases.get_object(Preposition::By).ok_or_else(|| eprintln!("expected 'by' phrase, but could not find it"))?;
    let obj = object.map_or_else(|| None, |date| date.expect_object()).ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    check_operand!(obj, by);
    Ok(format!("{:?} {:?}, {:?}", verb, obj, by))
}

fn gen_ins_call(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    // this code is specifying 'label' as the object, but in machine code, memory address also can be the object.
    let obj = preposition_phrases.get_object(Preposition::To).map_or_else(|| None, |date| date.expect_label()).ok_or_else(|| eprintln!("expected label, but could not find it"))?;
    Ok(format!("{:?} {:?}", verb, obj))
}
fn gen_ins_cmp(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let to = preposition_phrases.get_object(Preposition::To).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    let obj = object.map_or_else(|| None, |date| date.expect_object()).ok_or_else(|| eprintln!("expected object, but could not find it"))?;
    check_operand!(obj, to);
    Ok(format!("{:?} {:?}, {:?}", verb, obj, to))
}
// fn gen_ins_ret(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
//     let to = preposition_phrases.get_object(Preposition::To).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
//     let az = if let Some(ap) = preposition_phrases.get_object(Preposition::As) {
//         format!("{:?}", ap)
//     } else {
//         format!("")
//     };
//     Ok(format!("{:?}{} {:?}, {:?}", verb, az, to, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
// }
// fn gen_ins_leave(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
//     todo!();
// }
// fn gen_ins_nop(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
//     todo!();
// }
// fn gen_ins_syscall(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:PrepositionPhrases) -> Result<String> {
//     todo!();
// }
// fn gen_ins_hlt(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
//     assert!(check_intransitive(object, preposition_phrases)); // todo: assert to error handling
//     Ok(format!("{:?}", verb))
// }


fn gen_ins_iv(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    assert!(check_intransitive(object, preposition_phrases));
    Ok(format!("{:?}", verb))
}
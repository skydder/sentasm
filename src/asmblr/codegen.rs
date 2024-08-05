use super::{data::Verb, Code, Data, DataSet, Keyword, Loc, Preposition, PrepositionPhrases, Result};

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
    let to = preposition_phrases.get_object(Preposition::To).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    let az = if let Some(ap) = preposition_phrases.get_object(Preposition::As) {
        format!("{:?}", ap)
    } else {
        format!("")
    };
    Ok(format!("{:?}{} {:?}, {:?}", verb, az, to, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_sub(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let to = preposition_phrases.get_object(Preposition::From).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    let az = if let Some(ap) = preposition_phrases.get_object(Preposition::As) {
        format!("{:?}", ap)
    } else {
        format!("")
    };
    Ok(format!("{:?}{} {:?}, {:?}", verb, az, to, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_mul(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let to = preposition_phrases.get_object(Preposition::By).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    Ok(format!("{:?} {:?}, {:?}", verb, to, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_div(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    
    Ok(format!("{:?} {:?}", verb, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_mov(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let to = preposition_phrases.get_object(Preposition::To).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    let az = if let Some(ap) = preposition_phrases.get_object(Preposition::As) {
        format!("{:?}", ap)
    } else {
        format!("")
    };
    Ok(format!("{:?}{} {:?}, {:?}", verb, az, to, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_jmp(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    // println!("{:?}", preposition_phrases);
    let to = preposition_phrases.get_object(Preposition::To).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    let az = if let Some(ap) = preposition_phrases.get_object(Preposition::If) {
        format!("j{:?}", ap)
    } else {
        format!("jmp")
    };

    Ok(format!("{} {:?}", az, to))
}
fn gen_ins_and(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let to = preposition_phrases.get_object(Preposition::With).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    Ok(format!("{:?} {:?}, {:?}", verb, to, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_or(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let to = preposition_phrases.get_object(Preposition::With).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    Ok(format!("{:?} {:?}, {:?}", verb, to, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_xor(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let to = preposition_phrases.get_object(Preposition::With).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    Ok(format!("{:?} {:?}, {:?}", verb, to, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_not(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    Ok(format!("{:?} {:?}", verb, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_neg(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    Ok(format!("{:?} {:?}", verb, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_shr(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let to = preposition_phrases.get_object(Preposition::By).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    Ok(format!("{:?} {:?}, {:?}", verb, to, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_shl(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let to = preposition_phrases.get_object(Preposition::By).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    Ok(format!("{:?} {:?}, {:?}", verb, to, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_call(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    Ok(format!("{:?} {:?}", verb, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
}
fn gen_ins_cmp(verb: Verb, verb_loc: Loc, object: Option<DataSet>, preposition_phrases:&mut PrepositionPhrases) -> Result<String> {
    let to = preposition_phrases.get_object(Preposition::To).ok_or_else(|| eprintln!("expected 'to' phrase, but could not find it"))?;
    Ok(format!("{:?} {:?}, {:?}", verb, to, object.ok_or_else(|| eprintln!("expected object, but could not find it"))?))
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
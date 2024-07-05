use crate::data::sentence::Keyword;

use super::{AsmError, Object, Preposition, PrepositionPhrases, Sentence, Verb};

pub fn codegen(s: &mut Sentence) -> Result<String, AsmError> {
    match s {
        Sentence::Sentence {
            verb,
            prepositional_phrases,
            object: Some(obj),
        } => match verb {
            Verb::Add => add_instruction(obj, prepositional_phrases),
            Verb::Substract => sub_instruction(obj, prepositional_phrases),
            Verb::Multiply => mul_instruction(obj, prepositional_phrases),
            Verb::Divide => div_instruction(obj, prepositional_phrases),
            Verb::Move => mov_instruction(obj, prepositional_phrases),
            Verb::And | Verb::Or | Verb::Xor => logical_binary_instruction(verb, obj, prepositional_phrases),
            Verb::Not => not_instruction(obj, prepositional_phrases),
            Verb::Negate => negate_instruction(obj, prepositional_phrases),
            Verb::ShiftLeft => shl_instruction(obj, prepositional_phrases),
            Verb::ShiftRight => shr_instruction(obj, prepositional_phrases),
            Verb::Call => call_instruction(obj, prepositional_phrases),
            Verb::Compare => cmp_instruction(obj, prepositional_phrases),
            _ => Err(AsmError::SyntaxError(format!("something is wrong?"))),
        },
        Sentence::Sentence {
            verb,
            prepositional_phrases,
            object: None,
        } => vi_instructions(verb, prepositional_phrases),
        Sentence::LabelDefinition(l) => Ok(format!("{}:", l)),
        // _ => Err(AsmError::SyntaxError(format!("something is wrong"))),
    }
}

fn as_processer<'a>(pps: &mut PrepositionPhrases) -> Result<&'a str, AsmError> {
    match pps.phrases.remove(&Preposition::As) {
        Some(Object::Keyword(Keyword::DoublePrecisionFloat)) => Ok("sd"),
        Some(Object::Keyword(Keyword::SinglePrecisionFloat)) => Ok("ss"),
        None => Ok(""),
        _ => Err(AsmError::SyntaxError(format!("as only takes key word")))
    }
}

fn add_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    if !pps.phrases.contains_key(&Preposition::To) {
        Err(AsmError::SyntaxError(
            "add instruction needs to clause".to_string(),
        ))
    } else {
        let to = pps.phrases.remove(&Preposition::To).unwrap();
        let suffix = as_processer(pps)?;
        assert!(pps.phrases.is_empty());
        Ok(format!("\tadd{suffix} {dest}, {src}", suffix = suffix, dest = to, src = o))
    }
}
fn cmp_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    if !pps.phrases.contains_key(&Preposition::To) {
        Err(AsmError::SyntaxError(
            "add instruction needs to clause".to_string(),
        ))
    } else {
        let to = pps.phrases.remove(&Preposition::To).unwrap();
        let suffix = as_processer(pps)?;
        assert!(pps.phrases.is_empty());
        Ok(format!("\tcmp{suffix} {dest}, {src}", suffix = suffix, dest = o, src = to))
    }
}

fn sub_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    if !pps.phrases.contains_key(&Preposition::From) {
        Err(AsmError::SyntaxError(
            "sub instruction needs to clause".to_string(),
        ))
    } else {
        let from = pps.phrases.remove(&Preposition::From).unwrap();
        let suffix = as_processer(pps)?;
        assert!(pps.phrases.is_empty());
        Ok(format!("\tsub{suffix} {dest}, {src}", suffix = suffix, dest = from, src = o))
    }
}

fn mul_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    if !pps.phrases.contains_key(&Preposition::By) {
        Err(AsmError::SyntaxError(
            "mul instruction needs to clause".to_string(),
        ))
    } else {
        let by = pps.phrases.remove(&Preposition::By).unwrap();
        assert!(pps.phrases.is_empty());
        Ok(format!("\timul {dest}, {src}", dest = o, src = by))
    }
}

fn shr_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    if !pps.phrases.contains_key(&Preposition::By) {
        Err(AsmError::SyntaxError(
            "shr instruction needs to clause".to_string(),
        ))
    } else {
        let by = pps.phrases.remove(&Preposition::By).unwrap();
        assert!(pps.phrases.is_empty());
        Ok(format!("\tshr {dest}, {src}", dest = o, src = by))
    }
}

fn shl_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    if !pps.phrases.contains_key(&Preposition::By) {
        Err(AsmError::SyntaxError(
            "shl instruction needs to clause".to_string(),
        ))
    } else {
        let by = pps.phrases.remove(&Preposition::By).unwrap();
        assert!(pps.phrases.is_empty());
        Ok(format!("\tshl {dest}, {src}", dest = o, src = by))
    }
}

fn div_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    assert!(pps.phrases.is_empty());
    Ok(format!("\tidiv {dest}", dest = o))
}

fn not_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    assert!(pps.phrases.is_empty());
    Ok(format!("\tnot {dest}", dest = o))
}

fn call_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    assert!(pps.phrases.is_empty());
    Ok(format!("\tcall {dest}", dest = o))
}

fn negate_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    assert!(pps.phrases.is_empty());
    Ok(format!("\tneg {dest}", dest = o))
}

fn mov_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    if !pps.phrases.contains_key(&Preposition::To) {
        Err(AsmError::SyntaxError(
            "move instruction needs to clause".to_string(),
        ))
    } else {
        let to = pps.phrases.remove(&Preposition::To).unwrap();
        assert!(pps.phrases.is_empty());
        Ok(format!("\tmov {dest}, {src}", dest = to, src = o))
    }
}

fn jmp_if_processer<'a>(pps: &mut PrepositionPhrases) -> Result<&'a str, AsmError> {
    match pps.phrases.remove(&Preposition::If) {
        Some(Object::Keyword(Keyword::NE)) => Ok("jne"),
        Some(Object::Keyword(Keyword::E)) => Ok("je"),
        Some(Object::Keyword(Keyword::G)) => Ok("jg"),
        Some(Object::Keyword(Keyword::GE)) => Ok("jge"),
        Some(Object::Keyword(Keyword::L)) => Ok("jl"),
        Some(Object::Keyword(Keyword::LE)) => Ok("jle"),
        None => Ok("jmp"),
        _ => Err(AsmError::SyntaxError(format!("as only takes key word")))
    }
}

fn vi_instructions(v: &mut Verb, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    match v {
        Verb::Return => Ok(format!("\tret")),
        Verb::Leave => Ok(format!("\tleave")),
        Verb::NoOperation => Ok(format!("\tnop")),
        Verb::SystemCall => Ok(format!("\tsyscall")),
        Verb::Halt => Ok(format!("\thlt")),
        Verb::Jump => {
            let to = pps.phrases.remove(&Preposition::To).unwrap();
            let verb = jmp_if_processer(pps)?;
            assert!(pps.phrases.is_empty());
            Ok(format!("\t{verb} {dest}", dest = to))
        }
        _ => Err(AsmError::SyntaxError(format!("something is wrong"))),
    }
}

fn logical_binary_instruction(verb: &Verb, o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    let v = match verb {
        Verb::And => Some("and"),
        Verb::Or => Some("or"),
        Verb::Xor => Some("xor"),
        _ => None
    }.unwrap();
    if !pps.phrases.contains_key(&Preposition::With) {
        Err(AsmError::SyntaxError(
            "mul instruction needs to clause".to_string(),
        ))
    } else {
        let by = pps.phrases.remove(&Preposition::With).unwrap();
        assert!(pps.phrases.is_empty());
        Ok(format!("\t{verb} {dest}, {src}", verb = v, dest = o, src = by))
    }
}
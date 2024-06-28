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
            _ => Err(AsmError::SyntaxError(format!("something is wrong"))),
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

fn add_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    if !pps.phrases.contains_key(&Preposition::To) {
        Err(AsmError::SyntaxError(
            "add instruction needs to clause".to_string(),
        ))
    } else {
        let to = pps.phrases.remove(&Preposition::To).unwrap();
        assert!(pps.phrases.is_empty());
        Ok(format!("\tadd {dest}, {src}", dest = to, src = o))
    }
}

fn sub_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    if !pps.phrases.contains_key(&Preposition::From) {
        Err(AsmError::SyntaxError(
            "add instruction needs to clause".to_string(),
        ))
    } else {
        let from = pps.phrases.remove(&Preposition::From).unwrap();
        assert!(pps.phrases.is_empty());
        Ok(format!("\tsub {dest}, {src}", dest = from, src = o))
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

fn div_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    assert!(pps.phrases.is_empty());
    Ok(format!("\tidiv {dest}", dest = o))
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

fn vi_instructions(v: &mut Verb, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    assert!(pps.phrases.is_empty());
    match v {
        Verb::Return => Ok(format!("\tret")),
        Verb::Leave => Ok(format!("\tleave")),
        Verb::NoOperation => Ok(format!("\tnop")),
        Verb::SystemCall => Ok(format!("\tsyscall")),
        Verb::Halt => Ok(format!("\thlt")),
        _ => Err(AsmError::SyntaxError(format!("something is wrong"))),
    }
}

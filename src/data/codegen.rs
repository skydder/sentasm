use super::{AsmError, Object, Preposition, PrepositionPhrases, Sentence, Verb};

pub fn codegen(s: &mut Sentence) -> Result<String, AsmError> {
    if let Sentence::Sentence {
        verb,
        object,
        prepositional_phrases,
    } = s
    {
        match verb {
            Verb::Add => add_instruction(object, prepositional_phrases),
            Verb::Substract => sub_instruction(object, prepositional_phrases),
            Verb::Multiply => mul_instruction(object, prepositional_phrases),
            Verb::Divide => div_instruction(object, prepositional_phrases),
            Verb::Move => mov_instruction(object, prepositional_phrases),
        }
    } else if let Sentence::LabelDefinition(l) = s {
        Ok(format!("{}:", l))
    } else {
        Err(AsmError::SyntaxError(format!("something is wrong")))
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
        Ok(format!("add {dest}, {src}", dest = to, src = o))
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
        Ok(format!("sub {dest}, {src}", dest = from, src = o))
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
        Ok(format!("imul {dest}, {src}", dest = o, src = by))
    }
}

fn div_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    assert!(pps.phrases.is_empty());
    Ok(format!("idiv {dest}", dest = o))
}

fn mov_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    if !pps.phrases.contains_key(&Preposition::To) {
        Err(AsmError::SyntaxError(
            "move instruction needs to clause".to_string(),
        ))
    } else {
        let to = pps.phrases.remove(&Preposition::To).unwrap();
        assert!(pps.phrases.is_empty());
        Ok(format!("mov {dest}, {src}", dest = to, src = o))
    }
}

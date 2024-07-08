use crate::data::sentence::Keyword;

use super::{AsmError, Object, Preposition, PrepositionPhrases, Sentence, Verb};

macro_rules! check_if {
    ($cond: expr, $msg:expr) => {
        if $cond {
            Ok(())
        } else {
            Err(AsmError::SyntaxError($msg))
        }?
    };
}

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

fn as_processer<'a>(pps: &PrepositionPhrases) -> Result<&'a str, AsmError> {
    match pps.consume(Preposition::As) {
        Some(Object::Keyword(Keyword::DoublePrecisionFloat)) => Ok("sd"),
        Some(Object::Keyword(Keyword::SinglePrecisionFloat)) => Ok("ss"),
        None => Ok(""),
        _ => Err(AsmError::SyntaxError(format!("as only takes key word")))
    }
}

fn add_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    check_if!(pps.have(Preposition::To), "add instruction requires 'to' phrase".to_string());
    let to = pps.consume(Preposition::To).unwrap();
    let suffix = as_processer(pps)?;
    check_if!(pps.have_no_phrases(), "add instruction accepts just 'to' and 'as'".to_string());
    Ok(format!("\tadd{suffix} {dest}, {src}", suffix = suffix, dest = to, src = o))
}
fn cmp_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    check_if!(pps.have(Preposition::To), "compare instruction requires 'to' phrase".to_string());
    let to = pps.consume(Preposition::To).unwrap();
    let suffix = as_processer(pps)?;
    check_if!(pps.have_no_phrases(), "compare instruction accepts just 'to' and 'as'".to_string());
    Ok(format!("\tcmp{suffix} {dest}, {src}", suffix = suffix, dest = o, src = to))
}

fn sub_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    check_if!(pps.have(Preposition::From), "substract instruction requires 'from' phrase".to_string());
    let from = pps.consume(Preposition::From).unwrap();
    let suffix = as_processer(pps)?;
    check_if!(pps.have_no_phrases(), "substract instruction accepts just 'from' and 'as'".to_string());
    Ok(format!("\tsub{suffix} {dest}, {src}", suffix = suffix, dest = from, src = o))

}

fn mul_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    check_if!(pps.have(Preposition::By), "multiply instruction requires 'by' phrase".to_string());
    let by = pps.consume(Preposition::By).unwrap();
    check_if!(pps.have_no_phrases(), "multiply instruction accepts just 'by'".to_string());
    Ok(format!("\timul {dest}, {src}", dest = o, src = by))
}

fn shr_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    check_if!(pps.have(Preposition::By), "shift_right instruction requires 'By' phrase".to_string());
    let by = pps.consume(Preposition::By).unwrap();
    check_if!(pps.have_no_phrases(), "shift_right instruction accepts just 'by'".to_string());
    Ok(format!("\tshr {dest}, {src}", dest = o, src = by))
}

fn shl_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    check_if!(pps.have(Preposition::By), "shift_left instruction requires 'By' phrase".to_string());
    let by = pps.consume(Preposition::By).unwrap();
    check_if!(pps.have_no_phrases(), "shift_left instruction accepts just 'by'".to_string());
    Ok(format!("\tshl {dest}, {src}", dest = o, src = by))
}

fn div_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    check_if!(pps.have_no_phrases(), "divide instruction doesn't accept any phrases".to_string());
    Ok(format!("\tidiv {dest}", dest = o))
}

fn not_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    check_if!(pps.have_no_phrases(), "not instruction doesn't accept any phrases".to_string());
    Ok(format!("\tnot {dest}", dest = o))
}

fn call_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    check_if!(pps.have_no_phrases(), "call instruction doesn't accept any phrases".to_string());
    Ok(format!("\tcall {dest}", dest = o))
}

fn negate_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    check_if!(pps.have_no_phrases(), "negate instruction doesn't accept any phrases".to_string());
    Ok(format!("\tneg {dest}", dest = o))
}

fn mov_instruction(o: &Object, pps: &mut PrepositionPhrases) -> Result<String, AsmError> {
    check_if!(pps.have(Preposition::To), "move instruction requires 'to' phrase".to_string());
    let to = pps.consume(Preposition::To).unwrap();
    check_if!(pps.have_no_phrases(), "move instruction accepts just 'to'".to_string());
    Ok(format!("\tmov {dest}, {src}", dest = to, src = o))
}

fn jmp_if_processer<'a>(pps: &PrepositionPhrases) -> Result<&'a str, AsmError> {
    match pps.consume(Preposition::If) {
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
            let to = pps.consume(Preposition::To).unwrap();
            let verb = jmp_if_processer(pps)?;
            check_if!(pps.have_no_phrases(), "jump instruction accepts just 'to'".to_string());
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
    check_if!(pps.have(Preposition::With), "logical opeation instructions require 'With' phrase".to_string());
    let by = pps.consume(Preposition::With).unwrap();
    check_if!(pps.have_no_phrases(), "logical opeation instructions accept just 'with'".to_string());
    Ok(format!("\t{verb} {dest}, {src}", verb = v, dest = o, src = by))
}
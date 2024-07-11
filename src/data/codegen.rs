use crate::data::sentence::Keyword;

use super::{AsmError, Object, Preposition, PrepositionPhrases, Sentence, Verb, TokenLocation};

macro_rules! check_if {
    ($cond: expr, $loc: expr, $msg: expr) => {
        if $cond {
            Ok(())
        } else {
            Err(AsmError::SyntaxError::<'a>($loc, $msg))
        }?
    };
}

pub fn codegen<'a>(s:Sentence<'a>) -> Result<String, AsmError<'a>> {
    match s {
        Sentence::Sentence {
            verb,
            prepositional_phrases,
            object: Some(obj),
        } => match verb.0 {
            Verb::Add => add_instruction(verb.1, obj, prepositional_phrases),
            Verb::Substract => sub_instruction(verb.1, obj, prepositional_phrases),
            Verb::Multiply => mul_instruction(verb.1, obj, prepositional_phrases),
            Verb::Divide => div_instruction(verb.1, obj, prepositional_phrases),
            Verb::Move => mov_instruction(verb.1, obj, prepositional_phrases),
            Verb::And | Verb::Or | Verb::Xor => logical_binary_instruction(verb, obj, prepositional_phrases),
            Verb::Not => not_instruction(verb.1, obj, prepositional_phrases),
            Verb::Negate => negate_instruction(verb.1, obj, prepositional_phrases),
            Verb::ShiftLeft => shl_instruction(verb.1, obj, prepositional_phrases),
            Verb::ShiftRight => shr_instruction(verb.1, obj, prepositional_phrases),
            Verb::Call => call_instruction(verb.1, obj, prepositional_phrases),
            Verb::Compare => cmp_instruction(verb.1, obj, prepositional_phrases),
            _ => Err(AsmError::SyntaxError(verb.1, format!("something is wrong?"))),
        },
        Sentence::Sentence {
            verb,
            prepositional_phrases,
            object: None,
        } => vi_instructions(verb, prepositional_phrases),
        Sentence::LabelDefinition(l) => Ok(format!("{}:\n", l.0)),
        // _ => Err(AsmError::SyntaxError(format!("something is wrong"))),
    }
}

fn as_processer<'a>(pps: &PrepositionPhrases<'a>) -> Result<&'a str, AsmError<'a>> {
    match pps.consume(Preposition::As) {
        Some((Object::Keyword(Keyword::DoublePrecisionFloat), _)) => Ok("sd"),
        Some((Object::Keyword(Keyword::SinglePrecisionFloat), _)) => Ok("ss"),
        None => Ok(""),
        Some((_, loc)) => Err(AsmError::SyntaxError::<'a>(loc, format!("as only takes key word")))
    }
}

fn add_instruction<'a>(loc:TokenLocation<'a>, o: Box<(Object<'a>, TokenLocation<'a>)>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have(Preposition::To), loc, "add instruction requires 'to' phrase".to_string());
    let to = pps.consume(Preposition::To).unwrap().0;
    let suffix = as_processer(&pps)?;
    check_if!(pps.have_no_phrases(), loc, "add instruction accepts just 'to' and 'as'".to_string());
    Ok(format!("\tadd{suffix} {dest}, {src}\n", suffix = suffix, dest = to, src = o.0))
}
fn cmp_instruction<'a>(loc:TokenLocation<'a>, o: Box<(Object<'a>, TokenLocation<'a>)>, pps:  PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have(Preposition::To), loc, "compare instruction requires 'to' phrase".to_string());
    let to = pps.consume(Preposition::To).unwrap().0;
    let suffix = as_processer(&pps)?;
    check_if!(pps.have_no_phrases(), loc, "compare instruction accepts just 'to' and 'as'".to_string());
    Ok(format!("\tcmp{suffix} {dest}, {src}\n", suffix = suffix, dest = o.0, src = to))
}

fn sub_instruction<'a>(loc:TokenLocation<'a>, o: Box<(Object<'a>, TokenLocation<'a>)>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have(Preposition::From), loc, "substract instruction requires 'from' phrase".to_string());
    let from = pps.consume(Preposition::From).unwrap().0;
    let suffix = as_processer(&pps)?;
    check_if!(pps.have_no_phrases(), loc, "substract instruction accepts just 'from' and 'as'".to_string());
    Ok(format!("\tsub{suffix} {dest}, {src}\n", suffix = suffix, dest = from, src = o.0))

}

fn mul_instruction<'a>(loc:TokenLocation<'a>, o:Box<(Object<'a>, TokenLocation<'a>)>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have(Preposition::By), loc, "multiply instruction requires 'by' phrase".to_string());
    let by = pps.consume(Preposition::By).unwrap().0;
    check_if!(pps.have_no_phrases(), loc, "multiply instruction accepts just 'by'".to_string());
    Ok(format!("\timul {dest}, {src}\n", dest = o.0, src = by))
}

fn shr_instruction<'a>(loc:TokenLocation<'a>, o: Box<(Object<'a>, TokenLocation<'a>)>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have(Preposition::By), loc, "shift_right instruction requires 'By' phrase".to_string());
    let by = pps.consume(Preposition::By).unwrap().0;
    check_if!(pps.have_no_phrases(), loc, "shift_right instruction accepts just 'by'".to_string());
    Ok(format!("\tshr {dest}, {src}\n", dest = o.0, src = by))
}

fn shl_instruction<'a>(loc:TokenLocation<'a>, o: Box<(Object<'a>, TokenLocation<'a>)>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have(Preposition::By), loc, "shift_left instruction requires 'By' phrase".to_string());
    let by = pps.consume(Preposition::By).unwrap().0;
    check_if!(pps.have_no_phrases(), loc, "shift_left instruction accepts just 'by'".to_string());
    Ok(format!("\tshl {dest}, {src}\n", dest = o.0, src = by))
}

fn div_instruction<'a>(loc:TokenLocation<'a>, o: Box<(Object<'a>, TokenLocation<'a>)>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have_no_phrases(), loc, "divide instruction doesn't accept any phrases".to_string());
    Ok(format!("\tidiv {dest}\n", dest = o.0))
}

fn not_instruction<'a>(loc:TokenLocation<'a>, o: Box<(Object<'a>, TokenLocation<'a>)>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have_no_phrases(), loc, "not instruction doesn't accept any phrases".to_string());
    Ok(format!("\tnot {dest}\n", dest = o.0))
}

fn call_instruction<'a>(loc:TokenLocation<'a>, o: Box<(Object<'a>, TokenLocation<'a>)>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have_no_phrases(), loc, "call instruction doesn't accept any phrases".to_string());
    Ok(format!("\tcall {dest}\n", dest = o.0))
}

fn negate_instruction<'a>(loc:TokenLocation<'a>, o: Box<(Object<'a>, TokenLocation<'a>)>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have_no_phrases(), loc, "negate instruction doesn't accept any phrases".to_string());
    Ok(format!("\tneg {dest}\n", dest = o.0))
}

fn mov_instruction<'a>(loc:TokenLocation<'a>, o: Box<(Object<'a>, TokenLocation<'a>)>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have(Preposition::To), loc, "move instruction requires 'to' phrase".to_string());
    let to = pps.consume(Preposition::To).unwrap().0;
    check_if!(pps.have_no_phrases(), loc, "move instruction accepts just 'to'".to_string());
    Ok(format!("\tmov {dest}, {src}\n", dest = to, src = o.0))
}

fn jmp_if_processer<'a>(pps: &PrepositionPhrases<'a>) -> Result<&'a str, AsmError<'a>> {
    match pps.consume(Preposition::If) {
        Some((Object::Keyword(Keyword::NE), _)) => Ok("jne"),
        Some((Object::Keyword(Keyword::E), _)) => Ok("je"),
        Some((Object::Keyword(Keyword::G), _)) => Ok("jg"),
        Some((Object::Keyword(Keyword::GE), _)) => Ok("jge"),
        Some((Object::Keyword(Keyword::L), _)) => Ok("jl"),
        Some((Object::Keyword(Keyword::LE), _)) => Ok("jle"),
        None => Ok("jmp"),
        Some((_, loc)) => Err(AsmError::SyntaxError::<'a>(loc, format!("as only takes key word")))
    }
}

fn vi_instructions<'a>(v: Box<(Verb, TokenLocation<'a>)>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    match v.0 {
        Verb::Return => Ok(format!("\tret\n")),
        Verb::Leave => Ok(format!("\tleave\n")),
        Verb::NoOperation => Ok(format!("\tnop\n")),
        Verb::SystemCall => Ok(format!("\tsyscall\n")),
        Verb::Halt => Ok(format!("\thlt\n")),
        Verb::Jump => {
            let to = pps.consume(Preposition::To).unwrap().0;
            let verb = jmp_if_processer(&pps)?;
            check_if!(pps.have_no_phrases(), v.1, "jump instruction accepts just 'to'".to_string());
            Ok(format!("\t{verb} {dest}\n", dest = to))
        }
        _ => Err(AsmError::SyntaxError(v.1, format!("something is wrong"))),
    }
}

fn logical_binary_instruction<'a>(verb:  Box<(Verb, TokenLocation<'a>)>, o: Box<(Object<'a>, TokenLocation<'a>)>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    let v = match verb.0 {
        Verb::And => Some("and"),
        Verb::Or => Some("or"),
        Verb::Xor => Some("xor"),
        _ => None
    }.unwrap();
    check_if!(pps.have(Preposition::With), verb.1, "logical opeation instructions require 'With' phrase".to_string());
    let by = pps.consume(Preposition::With).unwrap().0;
    check_if!(pps.have_no_phrases(), verb.1, "logical opeation instructions accept just 'with'".to_string());
    Ok(format!("\t{verb} {dest}, {src}\n", verb = v, dest = o.0, src = by))
}
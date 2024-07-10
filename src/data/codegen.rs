use crate::data::sentence::Keyword;

use super::{AsmError, Object, Preposition, PrepositionPhrases, Sentence, Verb, TokenInfo, TokenLocation};

macro_rules! check_if {
    ($cond: expr, $loc: expr, $msg: expr) => {
        if $cond {
            Ok(())
        } else {
            Err(AsmError::SyntaxError::<'a>($loc, $msg))
        }?
    };
}

pub fn codegen<'a>(s:Sentence<'a, 'a>) -> Result<String, AsmError<'a>> {
    match s {
        Sentence::Sentence {
            verb,
            prepositional_phrases,
            object: Some(obj),
        } => match verb.data {
            Verb::Add => add_instruction(verb.location, obj, prepositional_phrases),
            Verb::Substract => sub_instruction(verb.location, obj, prepositional_phrases),
            Verb::Multiply => mul_instruction(verb.location, obj, prepositional_phrases),
            Verb::Divide => div_instruction(verb.location, obj, prepositional_phrases),
            Verb::Move => mov_instruction(verb.location, obj, prepositional_phrases),
            Verb::And | Verb::Or | Verb::Xor => logical_binary_instruction(verb, obj, prepositional_phrases),
            Verb::Not => not_instruction(verb.location, obj, prepositional_phrases),
            Verb::Negate => negate_instruction(verb.location, obj, prepositional_phrases),
            Verb::ShiftLeft => shl_instruction(verb.location, obj, prepositional_phrases),
            Verb::ShiftRight => shr_instruction(verb.location, obj, prepositional_phrases),
            Verb::Call => call_instruction(verb.location, obj, prepositional_phrases),
            Verb::Compare => cmp_instruction(verb.location, obj, prepositional_phrases),
            _ => Err(AsmError::SyntaxError(verb.location, format!("something is wrong?"))),
        },
        Sentence::Sentence {
            verb,
            prepositional_phrases,
            object: None,
        } => vi_instructions(verb, prepositional_phrases),
        Sentence::LabelDefinition(l) => Ok(format!("{}:", l.data)),
        // _ => Err(AsmError::SyntaxError(format!("something is wrong"))),
    }
}

fn as_processer<'a>(pps: PrepositionPhrases<'a>) -> Result<&'a str, AsmError<'a>> {
    match pps.consume(Preposition::As) {
        Some(TokenInfo {data:Object::Keyword(Keyword::DoublePrecisionFloat), location: _}) => Ok("sd"),
        Some(TokenInfo {data:Object::Keyword(Keyword::SinglePrecisionFloat), location: _}) => Ok("ss"),
        None => Ok(""),
        Some(TokenInfo {data:_, location: loc}) => Err(AsmError::SyntaxError::<'a>(loc, format!("as only takes key word")))
    }
}

fn add_instruction<'a>(loc:TokenLocation<'a>, o: TokenInfo<Object<'a>>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have(Preposition::To), loc, "add instruction requires 'to' phrase".to_string());
    let to = pps.consume(Preposition::To).unwrap().data;
    let suffix = as_processer(pps)?;
    check_if!(pps.have_no_phrases(), loc, "add instruction accepts just 'to' and 'as'".to_string());
    Ok(format!("\tadd{suffix} {dest}, {src}", suffix = suffix, dest = to, src = o.data))
}
fn cmp_instruction<'a>(loc:TokenLocation<'a>, o: TokenInfo<Object<'a>>, pps:  PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have(Preposition::To), loc, "compare instruction requires 'to' phrase".to_string());
    let to = pps.consume(Preposition::To).unwrap().data;
    let suffix = as_processer(pps)?;
    check_if!(pps.have_no_phrases(), loc, "compare instruction accepts just 'to' and 'as'".to_string());
    Ok(format!("\tcmp{suffix} {dest}, {src}", suffix = suffix, dest = o.data, src = to))
}

fn sub_instruction<'a>(loc:TokenLocation<'a>, o: TokenInfo<Object<'a>>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have(Preposition::From), loc, "substract instruction requires 'from' phrase".to_string());
    let from = pps.consume(Preposition::From).unwrap().data;
    let suffix = as_processer(pps)?;
    check_if!(pps.have_no_phrases(), loc, "substract instruction accepts just 'from' and 'as'".to_string());
    Ok(format!("\tsub{suffix} {dest}, {src}", suffix = suffix, dest = from, src = o.data))

}

fn mul_instruction<'a>(loc:TokenLocation<'a>, o:TokenInfo<Object<'a>>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have(Preposition::By), loc, "multiply instruction requires 'by' phrase".to_string());
    let by = pps.consume(Preposition::By).unwrap().data;
    check_if!(pps.have_no_phrases(), loc, "multiply instruction accepts just 'by'".to_string());
    Ok(format!("\timul {dest}, {src}", dest = o.data, src = by))
}

fn shr_instruction<'a>(loc:TokenLocation<'a>, o: TokenInfo<Object<'a>>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have(Preposition::By), loc, "shift_right instruction requires 'By' phrase".to_string());
    let by = pps.consume(Preposition::By).unwrap().data;
    check_if!(pps.have_no_phrases(), loc, "shift_right instruction accepts just 'by'".to_string());
    Ok(format!("\tshr {dest}, {src}", dest = o.data, src = by))
}

fn shl_instruction<'a>(loc:TokenLocation<'a>, o: TokenInfo<Object<'a>>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have(Preposition::By), loc, "shift_left instruction requires 'By' phrase".to_string());
    let by = pps.consume(Preposition::By).unwrap().data;
    check_if!(pps.have_no_phrases(), loc, "shift_left instruction accepts just 'by'".to_string());
    Ok(format!("\tshl {dest}, {src}", dest = o.data, src = by))
}

fn div_instruction<'a>(loc:TokenLocation<'a>, o: TokenInfo<Object<'a>>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have_no_phrases(), loc, "divide instruction doesn't accept any phrases".to_string());
    Ok(format!("\tidiv {dest}", dest = o.data))
}

fn not_instruction<'a>(loc:TokenLocation<'a>, o: TokenInfo<Object<'a>>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have_no_phrases(), loc, "not instruction doesn't accept any phrases".to_string());
    Ok(format!("\tnot {dest}", dest = o.data))
}

fn call_instruction<'a>(loc:TokenLocation<'a>, o: TokenInfo<Object<'a>>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have_no_phrases(), loc, "call instruction doesn't accept any phrases".to_string());
    Ok(format!("\tcall {dest}", dest = o.data))
}

fn negate_instruction<'a>(loc:TokenLocation<'a>, o: TokenInfo<Object<'a>>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have_no_phrases(), loc, "negate instruction doesn't accept any phrases".to_string());
    Ok(format!("\tneg {dest}", dest = o.data))
}

fn mov_instruction<'a>(loc:TokenLocation<'a>, o: TokenInfo<Object<'a>>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    check_if!(pps.have(Preposition::To), loc, "move instruction requires 'to' phrase".to_string());
    let to = pps.consume(Preposition::To).unwrap().data;
    check_if!(pps.have_no_phrases(), loc, "move instruction accepts just 'to'".to_string());
    Ok(format!("\tmov {dest}, {src}", dest = to, src = o.data))
}

fn jmp_if_processer<'a>(pps: PrepositionPhrases<'a>) -> Result<&'a str, AsmError<'a>> {
    match pps.consume(Preposition::If) {
        Some(TokenInfo{data: Object::Keyword(Keyword::NE), location: _}) => Ok("jne"),
        Some(TokenInfo{data: Object::Keyword(Keyword::E), location: _}) => Ok("je"),
        Some(TokenInfo{data: Object::Keyword(Keyword::G), location: _}) => Ok("jg"),
        Some(TokenInfo{data: Object::Keyword(Keyword::GE), location: _}) => Ok("jge"),
        Some(TokenInfo{data: Object::Keyword(Keyword::L), location: _}) => Ok("jl"),
        Some(TokenInfo{data: Object::Keyword(Keyword::LE), location: _}) => Ok("jle"),
        None => Ok("jmp"),
        Some(TokenInfo{data: _, location: loc}) => Err(AsmError::SyntaxError::<'a>(loc, format!("as only takes key word")))
    }
}

fn vi_instructions<'a>(v: TokenInfo<Verb>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    match v.data {
        Verb::Return => Ok(format!("\tret")),
        Verb::Leave => Ok(format!("\tleave")),
        Verb::NoOperation => Ok(format!("\tnop")),
        Verb::SystemCall => Ok(format!("\tsyscall")),
        Verb::Halt => Ok(format!("\thlt")),
        Verb::Jump => {
            let to = pps.consume(Preposition::To).unwrap().data;
            let verb = jmp_if_processer(pps)?;
            check_if!(pps.have_no_phrases(), v.location, "jump instruction accepts just 'to'".to_string());
            Ok(format!("\t{verb} {dest}", dest = to))
        }
        _ => Err(AsmError::SyntaxError(v.location, format!("something is wrong"))),
    }
}

fn logical_binary_instruction<'a>(verb:  TokenInfo<Verb>, o: TokenInfo<Object<'a>>, pps: PrepositionPhrases<'a>) -> Result<String, AsmError<'a>> {
    let v = match verb.data {
        Verb::And => Some("and"),
        Verb::Or => Some("or"),
        Verb::Xor => Some("xor"),
        _ => None
    }.unwrap();
    check_if!(pps.have(Preposition::With), verb.location, "logical opeation instructions require 'With' phrase".to_string());
    let by = pps.consume(Preposition::With).unwrap().data;
    check_if!(pps.have_no_phrases(), verb.location, "logical opeation instructions accept just 'with'".to_string());
    Ok(format!("\t{verb} {dest}, {src}", verb = v, dest = o.data, src = by))
}
use macros::match_data2;
use tokenizer::{emit_error, Location, Token, Tokenizer};
use data::{Data, DataSet, Immediate, Memory, Register};

use crate::{parse_data_set, parse_number, parse_register};

// base = reg
// index = reg
// scale = 1|2|4|8
// disp = num
// mem = '*(' ( ( base ( '+' index ( '*' scale )? )? ( '+' disp )? ')' ) 
//         | ( index '*' scale '+' )? disp ')' ) )

pub(crate) fn parse_memory<'a>(tokenizer: &'a Tokenizer<'a>) -> Option<(Memory<'a>, Location<'a>)> {
    let token = tokenizer.peek();
    let loc = tokenizer.get_location();
    
    if !token.get_punctuator().is_some_and(|punc| punc == "*(") {
        return None;
    }
    tokenizer.next();
    let mut mem = Memory::new();
    if tokenizer.peek().get_identifier().is_some_and(|reg| Register::is_reg(reg)) && !tokenizer.peek2().get_punctuator().is_some_and(|punc| punc == "*") {
        mem.set_base(parse_register(tokenizer).unwrap().0);
    }
    parse_mem_isd(&mut mem, tokenizer);
    tokenizer.expect_punctuator(")");
    Some((mem, loc))
}

fn parse_mem_isd<'a>(mem: &mut Memory<'a>, tokenizer: &'a Tokenizer<'a>) {
    if let Token::Identifier("+", _) = tokenizer.peek() {
        tokenizer.next();
    }
    if tokenizer.peek().get_identifier().is_some_and(|reg| Register::is_reg(reg)) {
        parse_mem_is(mem, tokenizer);
    }
    parse_mem_d(mem, tokenizer);
    parse_mem_size(mem, tokenizer);
}

fn parse_mem_is<'a>(mem: &mut Memory<'a>, tokenizer: &Tokenizer<'a>) {
    let parse_result = parse_register(tokenizer);
    if parse_result.is_none() {
        // error
        emit_error!(tokenizer.get_location(), "expected register as index, but could not find it.");
        todo!()
    }
    let (reg, s_loc) = parse_result.unwrap();
    mem.set_index(reg);
    if let Token::Identifier("*", _) = tokenizer.peek() {
        tokenizer.next();
        let scale: u8 = match parse_number(&tokenizer.peek()) {
            Some(1) => 1,
            Some(2) => 2,
            Some(4) => 4,
            Some(8) => 8,
            _ => {
                // error
                emit_error!(s_loc, "the value of scale should be 1, 2, 4, or 8.");
                todo!()
            }
        };
        tokenizer.next();
        mem.set_scale(scale);
    }
}

fn parse_mem_d<'a>(mem: &mut Memory<'a>, tokenizer: &'a Tokenizer<'a>) {
    let mut sign1 = false;
    if let Token::Identifier("+", _) = tokenizer.peek() {
        tokenizer.next();
    } else if let Token::Identifier("-", _) = tokenizer.peek() {
        tokenizer.next();
        sign1 = true;
    } else if let Token::Identifier("by", _) = tokenizer.peek() {
         return;
    }
    match parse_data_set(tokenizer) {
        Some(match_data2!(Immediate, Immediate(i, size, sign2), loc)) => {
            mem.set_disp(DataSet::new_(Data::Immediate(Immediate(i, size, sign1 || sign2)), loc));
        },
        Some(match_data2!(Label, label, loc)) => {
            mem.set_disp(DataSet::new_(Data::Label(label), loc));
        },
        other => {
            // error
            let loc = if let Some(data) = other {
                data.location
            } else {
                tokenizer.get_location()
            };
            emit_error!(loc, "expected immediate or label for displacement, but found other");
            todo!()
        }
    }
}

fn parse_mem_size<'a>(mem: &mut Memory<'a>, tokenizer: &Tokenizer<'a>) {
    if let Token::Identifier("by", _) = tokenizer.peek() {
        tokenizer.next();
        let size = match tokenizer.peek().get_identifier() {
            Some("8bit") => 8,
            Some("16bit") => 16,
            Some("32bit") => 32,
            Some("64bit") => 64,
            _ => {
                // error
                emit_error!(tokenizer.get_location(), "only size determiner can come here, but other is here");
                todo!()
            }
        };
        tokenizer.next();
        mem.set_size(size);
    }
}
use std::alloc::LayoutError;

use macros::match_data;
use tokenizer::{Location, Token, Tokenizer};
use data::{Data, DataSet, Immediate, Label, Memory, Register};

use crate::{parse_data_set, parse_immediate, parse_number, parse_register};
// base = reg
// index = reg
// scale = 1|2|4|8
// disp = num
// mem = '*(' ( ( base ( '+' index ( '*' scale )? )? ( '+' disp )? ')' ) 
//         | ( index '*' scale '+' )? disp ')' ) )

pub(crate) fn parse_memory<'a>(tokenizer: &Tokenizer<'a>) -> Option<(Memory<'a>, Location<'a>)> {
    let mut token = tokenizer.peek();
    let loc = tokenizer.get_location();
    
    if !token.get_punctuator().is_some_and(|punc| punc == "*(") {
        return None;
    }
    token = tokenizer.next();
    let mut mem = Memory::new();
    match token {
        Token::Identifier(ident, _) => {
            if Register::is_reg(ident) {

            } else {

            }
        },
        Token::Number(..) | Token::Punctuator("-", _) => {
            let disp = parse_immediate(tokenizer).unwrap();
            mem.set_disp(DataSet::new_(data::Data::Immediate(disp.0), disp.1));
            token = tokenizer.peek();
        },
        _ => {
            todo!()
        }
    }
    Some((mem, loc))
}

fn parse_mem_isd<'a>(mem: &mut Memory<'a>, tokenizer: &Tokenizer<'a>) {
    if let Token::Identifier("+", _) = tokenizer.peek() {
        tokenizer.next();
    }
    if tokenizer.peek().get_identifier().is_some_and(|reg| Register::is_reg(reg)) {
        parse_mem_is(mem, tokenizer);
    }
    parse_mem_d(mem, tokenizer);
}

fn parse_mem_is<'a>(mem: &mut Memory<'a>, tokenizer: &Tokenizer<'a>) {
    let parse_result = parse_register(tokenizer);
    if parse_result.is_none() {
        // error
        todo!()
    }
    let (reg, _) = parse_result.unwrap();
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
                todo!()
            }
        };
        tokenizer.next();
        mem.set_scale(scale);
    }
}

fn parse_mem_d<'a>(mem: &mut Memory<'a>, tokenizer: &Tokenizer<'a>) {
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
        match_data!(Immediate(i, size, sign2), loc) => {
            mem.set_disp(DataSet::new_(Data::Immediate(Immediate(i, size, sign1 || sign2)), loc));
        },
        match_data!(Label(label), loc) => {
            mem.set_disp(DataSet::new_(Data::Label(Label(label)), loc));
        },
        _ => {
            // error
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
                todo!()
            }
        };
        tokenizer.next();
        mem.set_size(size);
    }
}
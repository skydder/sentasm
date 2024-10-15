use data::{REG8, REG16,  REG32, REG64, RegType, Register};
use tokenizer::Tokenizer;

pub(crate) fn parse_register<'a>(tokenizer: &Tokenizer<'a>) -> Option<Register<'a>> {
    let token = tokenizer.peek();
    if !token.is_identifier() {
        return None;
    }
    let reg_name = token.get_identifier().unwrap();
    for (i, reg8) in REG8.into_iter().enumerate() {
        if reg_name == *reg8 {
            let reg_value = (i & 7) as u8;
            return Some(Register(reg8, 8, reg_value, i > 7, RegType::GP8));
        }
    }

    for (i, reg16) in REG16.into_iter().enumerate() {
        if reg_name == *reg16 {
            let reg_value = (i & 7) as u8;
            return Some(Register(reg16, 16, reg_value, i > 7, RegType::GP16));
        }
    }

    for (i, reg32) in REG32.into_iter().enumerate() {
        if reg_name == *reg32 {
            let reg_value = (i & 7) as u8;
            return Some(Register(reg32, 32, reg_value, i > 7, RegType::GP32));
        }
    }

    for (i, reg64) in REG64.into_iter().enumerate() {
        if reg_name == *reg64 {
            let reg_value = (i & 7) as u8;
            return Some(Register(reg64, 64, reg_value, i > 7, RegType::GP64));
        }
    }
    None
}
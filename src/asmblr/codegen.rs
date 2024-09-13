use std::hash::BuildHasher;

use super::{
    codegen_verb, parser::Sentence, Code, Data, DataSet, Loc, Memory, Preposition, PrepositionPhrases, Result, Verb, Register
};

pub fn codegen(code: Code, asm: &mut String) -> Result<()> {
    let line = match code {
        Code::NullStmt => Ok(format!("")),
        Code::LabelDef(l) => Ok(format!("{}:\n", l.0)),
        Code::Section(l) => Ok(format!("section {}\n", l.0)),
        Code::Sentence(sentense) => Ok(format!(
            "\t{}\n",
            codegen_sentence(sentense)?
        )),
    }?;
    asm.push_str(&line);
    Ok(())
}

fn codegen_sentence(
    sentence: Sentence
) -> Result<String> {
    match &sentence.verb {
        Verb("define") => gen_ins_def(sentence),
        Verb("globalize") => gen_ins_global(sentence),
        Verb("allocate") => gen_ins_alloc(sentence),
        Verb("extern") => gen_ins_extern(sentence),
        _ => codegen_verb(sentence)
    }
}

fn gen_ins_def(
    sentence: Sentence
) -> Result<String> {
    let obj = sentence.preposition_phrases
        .get_object(Preposition("obj"))
        .map_or_else(|| None, |date| date.expect_label())
        .ok_or_else(|| eprintln!("expected label, but could not find it"))?;

    let az = sentence.preposition_phrases
        .get_object(Preposition("as"))
        .map_or_else(|| None, |date| date.expect_define())
        .ok_or_else(|| eprintln!("expected 'as' phrase, but could not find it"))?;
    let by = sentence.preposition_phrases
        .get_object(Preposition("by"))
        .map_or_else(|| None, |date| date.expect_keyword())
        .ok_or_else(|| eprintln!("expected 'by' phrase, but could not find it"))?;

    match (obj.data, az.data, by.data) {
        (Data::Label(l), Data::Define(i), Data::Keyword(super::data::Keyword("8bit"))) => {
            Ok(format!("{} db {:?}", l.0, i))
        }
        (Data::Label(l), Data::Define(i), Data::Keyword(super::data::Keyword("16bit"))) => {
            Ok(format!("{} dw {:?}", l.0, i))
        }
        (Data::Label(l), Data::Define(i), Data::Keyword(super::data::Keyword("32bit"))) => {
            Ok(format!("{} dd {:?}", l.0, i))
        }
        (Data::Label(l), Data::Define(i), Data::Keyword(super::data::Keyword("64bit"))) => {
            Ok(format!("{} dq {:?}", l.0, i))
        }
        _ => todo!(),
    }
}

fn gen_ins_global(
    sentence: Sentence
) -> Result<String> {
    let obj = sentence.preposition_phrases
        .get_object(Preposition("obj"))
        .map_or_else(|| None, |date| date.expect_label())
        .ok_or_else(|| eprintln!("expected label, but could not find it"))?;

    Ok(format!("global {:?}", obj))
}

fn gen_ins_alloc(
    sentence: Sentence
) -> Result<String> {
    let obj = sentence.preposition_phrases
        .get_object(Preposition("obj"))
        .map_or_else(|| None, |date| date.expect_label())
        .ok_or_else(|| eprintln!("expected label, but could not find it"))?;

    let vor: DataSet = sentence.preposition_phrases
        .get_object(Preposition("for"))
        .map_or_else(|| None, |date| date.expect_immediate())
        .ok_or_else(|| eprintln!("expected 'for' phrase, but could not find it"))?;
    let by = sentence.preposition_phrases
        .get_object(Preposition("by"))
        .map_or_else(|| None, |date| date.expect_keyword())
        .ok_or_else(|| eprintln!("expected 'by' phrase, but could not find it"))?;

    match (obj.data, vor.data, by.data) {
        (Data::Label(l), Data::Immediate(i), Data::Keyword(super::data::Keyword("8bit"))) => {
            Ok(format!("{} resb {}", l.0, i.0))
        }
        (Data::Label(l), Data::Immediate(i), Data::Keyword(super::data::Keyword("16bit"))) => {
            Ok(format!("{} resw {}", l.0, i.0))
        }
        (Data::Label(l), Data::Immediate(i), Data::Keyword(super::data::Keyword("32bit"))) => {
            Ok(format!("{} resd {}", l.0, i.0))
        }
        (Data::Label(l), Data::Immediate(i), Data::Keyword(super::data::Keyword("64bit"))) => {
            Ok(format!("{} resq {}", l.0, i.0))
        }
        _ => todo!(),
    }
}

fn gen_ins_extern(
    sentence: Sentence
) -> Result<String> {
    let obj = sentence.preposition_phrases
        .get_object(Preposition("obj"))
        .map_or_else(|| None, |date| date.expect_label())
        .ok_or_else(|| eprintln!("expected label, but could not find it"))?;

    Ok(format!("extern {:?}", obj))
}



pub fn sib_raw(scale: u8, index: u8, base: u8) -> u8 {
    sib_scale(scale) << 6 | index << 3 | base
}

fn sib_scale(scale: u8) -> u8 {
    match scale {
        1 => 0b00,
        2 => 0b01,
        4 => 0b10,
        8 => 0b11,
        _ => panic!("unexpected!!")
    }
}

pub fn mod_rm_raw(mode: u8, reg: u8, rm: u8) -> u8 {
    mode << 6 | reg << 3 | rm
}

fn instruction(opcode: Vec<u8>, prefix: Vec<u8>, mod_rm: u8, disp: Vec<u8>, imm: Vec<u8>) -> Vec<u8> {
    todo!()
}

fn put_byte(byte: u8) -> String {
    format!("db {:#02x}\n", byte)
}

impl<'a> Memory<'a> {
    pub fn mode(&self) -> (u8, u32) {
        let disp = self.disp_size;
        if disp == 0 {
            (0b00, 0)
        } else if disp == 8 {
            (0b01, self.disp())
        } else if disp == 32 {
            (0b10, self.disp())
        } else {
            panic!("invalid memory form for mod_rm")
        }
    }

    pub fn disp(&self) -> u32 {
        todo!()
    }

    pub fn mod_rm(&self, reg: u8) -> (u8, u8, u8, u32) {
        let mut rex = Rex::new(); 
        let (mut mode, mut disp) = self.mode();
        let mut sib: u8 = 0;
        let mut rm: u8 = match self.base {
            Some(Register(_, 64, 4, b)) => todo!(),
            Some(Register(_, 64, 5, b)) => {
                mode = 0b10;
                disp = 0;
                rex.rex_b(b);
                5
            },
            Some(Register(_, 64, i, b)) => { 
                rex.rex_b(b);
                i
            },
            None => todo!(),
            _ => todo!()
        };
        if self.index.is_some() {
            let idx = self.index.unwrap();
            let scl = self.scale.unwrap_or(1);
            rm = 4;
            rex.rex_w(idx.3);
            if let Some(base) = self.base {
                rex.rex_b(base.3);
                sib = sib_raw(scl, idx.2, base.2);
            } else {
                sib = sib_raw(scl, idx.2, 0b101);
            }
        }
        (rex.generate(), mod_rm_raw(mode, reg, rm), sib, disp)
    }

}

fn mod_rm(reg: DataSet, rm: DataSet) -> u8 {
    let reg_r = reg.get_register().unwrap();
    
    if rm.is_register() {
        mod_rm_raw(0b11, reg_r.2, rm.get_register().unwrap().2)
    } else {
        todo!()
    }
}

struct ModRM {
    mode: u8,
    reg: u8,
    rm: u8,
}

impl ModRM {
    fn new() -> Self {
        Self { mode: 0, reg: 0, rm: 0 }
    }
    
    fn set_rm(&mut self, rm: u8) {
        self.rm = rm;
    }

    fn set_reg(&mut self, reg: u8) {
        self.reg = reg;
    }

    fn set_mode(&mut self, mode: u8) {
        self.mode = mode;
    }

    fn generate(self) -> u8 {
        self.mode << 6 | self.reg << 3 | self.rm
    }
}

struct SIB {
    scale: u8,
    index: u8,
    base: u8,
}

impl SIB {
    fn new() -> Self {
        Self { scale: 0, index: 0, base: 0 }
    }
    
    fn set_scale(&mut self, scale: u8) {
        self.scale = match scale {
            1 => 0b00,
            2 => 0b01,
            4 => 0b10,
            8 => 0b11,
            _ => panic!("unexpected scale")
        };
    }

    fn set_index(&mut self, index: u8) {
        self.index = index;
    }

    fn set_base(&mut self, base: u8) {
        self.base = base;
    }

    fn generate(self) -> u8 {
        self.scale << 6 | self.index << 3 | self.base
    }
}

struct Rex {
   //rex: bool, //= 0x40,
    w: bool, //= 0b1000,
    r: bool, 
    x: bool, //= 0b0010, // extention of the SIB index field
    b: bool, //= 0b0001, // extention of the ModR/M r/m field, SIB base field, opcode reg field
}

impl Rex {
    fn new() -> Self {
        Self { w: false, r: false, x: false, b: false }
    }

    fn rex(w: bool, r: bool, x: bool, b: bool) -> Self {
        Self { w, r, x, b }
    }

    fn rex_w(&mut self, w: bool) {
        self.w = w;
    }

    // extention of the ModR/M of reg field
    fn rex_r(&mut self, r: bool) {
        self.r = r;
    }

    // extention of the SIB index field
    fn rex_x(&mut self, x: bool) {
        self.x = x;
    }

    // extention of the ModR/M r/m field, SIB base field, opcode reg field
    fn rex_b(&mut self, b: bool) {
        self.b = b;
    }

    fn generate(self) -> u8{
        let mut rex = 0b1000 * (self.w as u8) | 0b0100 * (self.r as u8) | 0b0010 * (self.x as u8) | 0b0001 * (self.b as u8);
        if rex != 0 {
            0x40 | rex
        } else {
            0
        }
    }
}

enum Disp {
    Disp8(i8),
    Disp16(i16),
    Disp32(i32),
    None
}

struct Builder {
    rex: Rex,
    mod_rm: ModRM,
    sib: SIB,
    disp: Disp
}

impl Builder {
    fn new() -> Self {
        Self { rex: Rex::new(), mod_rm: ModRM::new(), sib: SIB::new(), disp: Disp::None }
    }

    fn set_rm_reg(&mut self, reg: Register) {
        self.mod_rm.set_mode(0b11);
        self.mod_rm.set_rm(reg.2);
        self.rex.rex_b(reg.3);
    }

    fn set_reg_reg(&mut self, reg: Register) {
        self.mod_rm.set_reg(reg.2);
        self.rex.rex_r(reg.3);
    }

    fn set_reg_op(&mut self, op: u8) {
        self.mod_rm.set_reg(op);
    }

    fn set_rm_mem(&mut self, mem: Memory) {
        match mem.disp_size {
            0 => self.mod_rm.set_mode(0b00),
            8 => {
                self.mod_rm.set_mode(0b01);
                // self.disp = Disp::Disp8(mem.displacement.unwrap().)
            },
            16 | 32 => {
                self.mod_rm.set_mode(0b11)
            },
            _ => todo!()
        }

        if mem.index.is_some() {
            self.mod_rm.set_rm(0b100);

            let index = mem.index.unwrap();
            let scale = mem.scale.unwrap_or(1);
            
            self.sib.set_index(index.2);
            self.rex.rex_x(index.3);
            self.sib.set_scale(scale);
            
            if let Some(Register(_, 64, base, b)) = mem.base {
                self.sib.set_base(base);
                self.rex.rex_b(b);
            } else {
                self.sib.set_base(0b101);
                self.rex.rex_b(false);
            }
            return;
        }

        match mem.base {
            Some(Register(_, 64, 4, _)) => todo!(),
            Some(Register(_, 64, 5, _)) => todo!(),
            Some(Register(_, 64, rm, b)) => {
                self.rex.rex_b(b);
                self.mod_rm.set_rm(rm);
            },
            None => todo!(),
            _ => todo!()
        }


    }
}
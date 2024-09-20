use super::{
    codegen_verb, Sentence, Code, Data, DataSet, Immediate, Memory, Preposition, Register, Result, Verb, Keyword
};

use macros::match_data;

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
        (Data::Label(l), Data::Define(i), Data::Keyword(Keyword("8bit"))) => {
            Ok(format!("{} db {}", l.0, i))
        }
        (Data::Label(l), Data::Define(i), Data::Keyword(Keyword("16bit"))) => {
            Ok(format!("{} dw {}", l.0, i))
        }
        (Data::Label(l), Data::Define(i), Data::Keyword(Keyword("32bit"))) => {
            Ok(format!("{} dd {}", l.0, i))
        }
        (Data::Label(l), Data::Define(i), Data::Keyword(Keyword("64bit"))) => {
            Ok(format!("{} dq {}", l.0, i))
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

    Ok(format!("global {}", obj))
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
        (Data::Label(l), Data::Immediate(i), Data::Keyword(Keyword("8bit"))) => {
            Ok(format!("{} resb {}", l.0, i.0))
        }
        (Data::Label(l), Data::Immediate(i), Data::Keyword(Keyword("16bit"))) => {
            Ok(format!("{} resw {}", l.0, i.0))
        }
        (Data::Label(l), Data::Immediate(i), Data::Keyword(Keyword("32bit"))) => {
            Ok(format!("{} resd {}", l.0, i.0))
        }
        (Data::Label(l), Data::Immediate(i), Data::Keyword(Keyword("64bit"))) => {
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

    Ok(format!("extern {}", obj))
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

    pub fn generate(self, flag: bool) -> u8{
        let mut rex = 0b1000 * (self.w as u8) | 0b0100 * (self.r as u8) | 0b0010 * (self.x as u8) | 0b0001 * (self.b as u8);
        if rex != 0 || flag{
            0x40 | rex
        } else {
            0
        }
    }
}

pub enum Disp {
    Disp8(i8),
    Disp16(i16),
    Disp32(i32),
    None
}

struct Builder {
    rex: Option<Rex>,
    mod_rm: Option<ModRM>,
    sib: Option<SIB>,
    disp: Disp,
    imm: Option<Immediate>
}

impl Builder {
    fn new(rex: Option<Rex>) -> Self {
        Self { rex: rex, mod_rm: None, sib: None, disp: Disp::None, imm: None }
    }

    // fn custom(rex: Option<Rex>, mod_rm: ModRM, )

    fn set_rex_b(&mut self, b: bool) {
        let mut rex = self.rex.clone().unwrap_or_else(|| Rex::new());
        rex.rex_b(b);
        self.rex = Some(rex)
    }

    fn set_rex_r(&mut self, r: bool) {
        let mut rex = self.rex.clone().unwrap_or_else(|| Rex::new());
        rex.rex_r(r);
        self.rex = Some(rex)
    }

    fn set_rex_x(&mut self, x: bool) {
        let mut rex = self.rex.clone().unwrap_or_else(|| Rex::new());
        rex.rex_x(x);
        self.rex = Some(rex)
    }

    fn set_rex_w(&mut self, w: bool) {
        let mut rex = self.rex.clone().unwrap_or_else(|| Rex::new());
        rex.rex_w(w);
        self.rex = Some(rex)
    }
    
    fn set_sib_base(&mut self, base: u8) {
        let mut sib = self.sib.clone().unwrap_or_else(|| SIB::new());
        sib.set_base(base);
        self.sib = Some(sib);
    }

    fn set_sib_index(&mut self, index: u8) {
        let mut sib = self.sib.clone().unwrap_or_else(|| SIB::new());
        sib.set_index(index);
        self.sib = Some(sib);
    }

    fn set_sib_scale(&mut self, scale: u8) {
        let mut sib = self.sib.clone().unwrap_or_else(|| SIB::new());
        sib.set_scale(scale);
        self.sib = Some(sib);
    }

    fn set_mod_rm_mod(&mut self, mode: u8) {
        let mut mod_rm = self.mod_rm.clone().unwrap_or_else(|| ModRM::new());
        mod_rm.set_mode(mode);
        self.mod_rm = Some(mod_rm);
    }

    fn set_mod_rm_reg(&mut self, reg: u8) {
        let mut mod_rm = self.mod_rm.clone().unwrap_or_else(|| ModRM::new());
        mod_rm.set_reg(reg);
        self.mod_rm = Some(mod_rm);
    }

    fn set_mod_rm_rm(&mut self, rm: u8) {
        let mut mod_rm = self.mod_rm.clone().unwrap_or_else(|| ModRM::new());
        mod_rm.set_rm(rm);
        self.mod_rm = Some(mod_rm);
    }

    fn set_disp_8(&mut self, disp: i8) {
        self.disp = Disp::Disp8(disp);
    }

    fn set_disp_32(&mut self, disp: i32) {
        self.disp = Disp::Disp32(disp);
    }

    fn set_disp_None(&mut self) {
        self.disp = Disp::None;
    }

    // #####################################
    fn set_rm_reg(&mut self, reg: Register) {
        self.set_mod_rm_mod(0b11);
        self.set_mod_rm_rm(reg.2);
        self.set_rex_b(reg.3);
    }

    fn set_reg_reg(&mut self, reg: Register) {
        self.set_mod_rm_reg(reg.2);
        self.set_rex_r(reg.3);
    }

    fn set_reg_op(&mut self, op: u8) {
        self.set_mod_rm_reg(op);
    }

    fn set_rm_mem(&mut self, mem: Memory) {
        match mem.disp_size {
            0 => self.set_mod_rm_mod(0b00),
            8 => {
                self.set_mod_rm_mod(0b01);
                // self.disp = Disp::Disp8(mem.displacement.unwrap().)
            },
            16 | 32 => {
                self.set_mod_rm_mod(0b11)
            },
            _ => todo!()
        }

        // [<base> '+' <index> ('*' <scale>)*]
        if mem.index.is_some() {
            self.set_mod_rm_mod(0b100);

            let index = mem.index.unwrap();
            let scale = mem.scale.unwrap_or(1);
            
            self.set_sib_index(index.2);
            self.set_rex_x(index.3);
            self.set_sib_scale(scale);
            
            if let Some(Register(_, 64, base, b)) = mem.base {
                self.set_sib_base(base);
                self.set_rex_b(b);
            } else {
                self.set_sib_base(0b101);
                self.set_rex_b(false);
            }
            return;
        }

        match mem.base {
            Some(Register(_, 64, 4, b)) => {
                todo!()
            },
            Some(Register(_, 64, 5, b)) => {
                if mem.disp_size == 0 {
                    self.set_rex_b(b);
                    self.set_mod_rm_mod(0b01);
                    self.set_disp_8(0);
                } else {
                    self.set_rex_b(b);
                    self.set_mod_rm_rm(5);
                }
            },
            Some(Register(_, 64, rm, b)) => {
                self.set_rex_b(b);
                self.set_mod_rm_rm(rm);
            },
            None => todo!(),
            _ => todo!()
        }
    }

    fn generate(self, rex: bool) -> (Option<u8>, Option<u8>, Option<u8>, Disp, Option<Immediate>) {
        (self.rex.map(|rex_l|rex_l.generate(rex)), self.mod_rm.map(|mod_rm| mod_rm.generate()), self.sib.map(|sib|sib.generate()), self.disp, self.imm)
    }
}

fn put_byte(byte: Byte) -> String {
    format!("db {:#02x}\n", byte)
}

type Byte = u8;

pub fn generate_machine_code(prefixes: Vec<Byte>, rex_prefix: Option<Byte>, opcode: Vec<Byte>, mod_rm: Option<Byte>, sib: Option<Byte>, disp: Option<Disp>, imm: Option<Immediate>) -> String {
    let mut code: String = String::new();
    for byte in prefixes {
        code += &put_byte(byte);
    }
    if let Some(rex) = rex_prefix {
        code += &put_byte(rex);
    }
    for byte in opcode {
        code += &put_byte(byte);
    }
    if let Some(modrm) = mod_rm {
        code += &put_byte(modrm);
    }
    if let Some(sib_b) = sib {
        code += &put_byte(sib_b);
    }
    if let Some(modrm) = mod_rm {
        code += &put_byte(modrm);
    }
    // not yet implemented
    code
}

// when you use this function, you have to convert \0..\7 to Register 
pub fn operands<'a>(rex:Option<Rex>, reg: Option<DataSet<'a>>, rm: Option<DataSet<'a>>, imm: Option<DataSet<'a>>, op4: Option<DataSet<'a>>) -> Builder {
    match (&reg, &rm, &imm, &op4) {
        (None, None, None, None) => Builder::new(rex),
        (match_data!(Register(_, 64, _, _)), match_data!(Register(_, 64, _, _)), None, None) => {
            let mut builder = Builder::new(rex);
            builder.set_reg_reg(reg.unwrap().get_register().unwrap());
            builder.set_rm_reg(rm.unwrap().get_register().unwrap());
            builder
        },

        _ => todo!()
    }
}
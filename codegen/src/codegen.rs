use crate::codegen_mc::emit_mc;

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
    fn w()  -> Self {
        Self { w: true, r: false, x: false, b: false }
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


pub struct Instruction {
    op_code: Vec<u8>,
    prefixes: Vec<u8>,
    rex: Option<Rex>,
    mod_rm: Option<ModRM>,
    sib: Option<SIB>,
    disp: Option<Immediate>,
    imm: Option<Immediate>
}

impl Instruction {
    pub fn new() -> Self {
        Self { op_code: Vec::new(), prefixes: Vec::new(), rex: None, mod_rm: None, sib: None, disp: None, imm: None }
    }

    pub fn set_rex(&mut self) {
        self.rex = Some(self.rex.clone().unwrap_or_else(|| Rex::new()));
    }

    fn set_rex_b(&mut self, b: bool) {
        if b {
            let mut rex = self.rex.clone().unwrap_or_else(|| Rex::new());
            rex.rex_b(b);
            self.rex = Some(rex)
        }
    }

    fn set_rex_r(&mut self, r: bool) {
        if r {
            let mut rex = self.rex.clone().unwrap_or_else(|| Rex::new());
            rex.rex_r(r);
            self.rex = Some(rex)
        }
    }

    fn set_rex_x(&mut self, x: bool) {
        if x {
            let mut rex = self.rex.clone().unwrap_or_else(|| Rex::new());
            rex.rex_x(x);
            self.rex = Some(rex)
        }
    }

    pub fn set_rex_w(&mut self) {
        let mut rex = self.rex.clone().unwrap_or_else(|| Rex::new());
        rex.rex_w(true);
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

    pub fn set_mod_rm_reg(&mut self, reg: u8) {
        let mut mod_rm = self.mod_rm.clone().unwrap_or_else(|| ModRM::new());
        mod_rm.set_reg(reg);
        self.mod_rm = Some(mod_rm);
    }

    fn set_mod_rm_rm(&mut self, rm: u8) {
        let mut mod_rm = self.mod_rm.clone().unwrap_or_else(|| ModRM::new());
        mod_rm.set_rm(rm);
        self.mod_rm = Some(mod_rm);
    }

    fn _set_disp(&mut self, disp: Immediate) {
        self.disp = Some(disp);
    }

    fn _set_imm(&mut self, imm: Immediate) {
        self.imm = Some(imm);
    }

    pub fn set_prefix(&mut self, prefix: u8) {
        self.prefixes.push(prefix);
    }

    pub fn set_opcode(&mut self, opcode: u8) {
        self.op_code.push(opcode);
    }

    fn _set_opecode_with_register(&mut self, opcode: u8, reg: Register) {
        self.set_opcode(opcode + reg.2);
    }

    // #####################################
    fn _set_reg(&mut self, reg: Register) {
        self.set_mod_rm_reg(reg.2);
        self.set_rex_r(reg.3);
    }

    fn _set_rm_reg(&mut self, reg: Register) {
        self.set_mod_rm_mod(0b11);
        self.set_mod_rm_rm(reg.2);
        self.set_rex_b(reg.3);
    }

    fn _set_rm_mem(&mut self, mem: Memory) {
        match mem.disp_size {
            0 => self.set_mod_rm_mod(0b00),
            8 => {
                self.set_mod_rm_mod(0b01);
                self._set_disp(mem.displacement.unwrap().get_immediate().unwrap());
            },
            16 | 32 => {
                self.set_mod_rm_mod(0b11);
                self._set_disp(mem.displacement.unwrap().get_immediate().unwrap());
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
                    self._set_disp(Immediate(0, 8, false));
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

    pub fn set_reg(&mut self, reg: DataSet) {
        match reg.data {
            Data::Register(r) => {
                self._set_reg(r);
            }
            _ => todo!("going to be error")
        }
    }

    pub fn set_rm(&mut self, rm: DataSet) {
        match rm.data {
            Data::Register(r) => {
                self._set_rm_reg(r);
            }
            Data::Memory(m) => {
                self._set_rm_mem(m);
            }
            _ => todo!("going to be error")
        }
    }

    pub fn set_imm(&mut self, imm: DataSet) {
        match imm.data {
            Data::Immediate(i) => {
                self._set_imm(i);
            }
            _ => todo!("going to be error")
        }
    }
    pub fn set_disp(&mut self, imm: DataSet) {
        match imm.data {
            Data::Immediate(i) => {
                self._set_disp(i);
            }
            _ => todo!("going to be error")
        }
    }
    pub fn set_opecode_with_register(&mut self, opcode: u8, reg: DataSet) {
        match reg.data {
            Data::Register(r) => {
                self._set_opecode_with_register(opcode, r);
            }
            _ => todo!("going to be error")
        }
    }

    fn emit_machine_code(mut self) -> Vec<u8> {
        let mut mc: Vec<u8> = Vec::new();
        mc.append(&mut self.prefixes);
        if self.rex.is_some() {
            mc.push(self.rex.unwrap().generate(true));
        }
        mc.append(&mut self.op_code);
        if self.mod_rm.is_some() {
            mc.push(self.mod_rm.unwrap().generate());
        }
        if self.sib.is_some() {
            mc.push(self.sib.unwrap().generate());
        }
        if self.disp.is_some() {
            mc.append(&mut self.disp.unwrap().generate());
        }
        if self.imm.is_some() {
            mc.append(&mut self.imm.unwrap().generate());
        }
        mc
    }
}

// when you use this function, you have to convert \0..\7 to Register 
pub fn operands<'a>(mut ins: Instruction, rex: Option<Rex>, reg: Option<DataSet<'a>>, rm: Option<DataSet<'a>>, imm: Option<DataSet<'a>>, op4: Option<DataSet<'a>>) -> Instruction {
    match (&reg, &rm, &imm, &op4) {
        (None, None, None, None) => ins,
        (match_data!(Register(_, 64, _, _)), match_data!(Register(_, 64, _, _)), None, None) => {
            ins.set_reg(reg.unwrap());
            ins.set_rm(rm.unwrap());
            ins
        },
        (match_data!(Register(_, 64, _, _)), match_data!(Memory{..}), None, None) => {
            ins.set_reg(reg.unwrap());
            ins.set_rm(rm.unwrap());
            ins
        },
        (match_data!(Register(_, 0, _, _)), match_data!(Memory{..}), match_data!(Immediate(..)), None) => {
            ins.set_rm(rm.unwrap());
            ins.set_reg(reg.unwrap());
            ins.set_imm(imm.unwrap());
            ins
        },
        (match_data!(Register(_, 0, _, _)), match_data!(Register(_, 64, _, _)), match_data!(Immediate(..)), None) => {
            ins.set_rm(rm.unwrap());
            ins.set_reg(reg.unwrap());
            ins.set_imm(imm.unwrap());
            ins
        }
        _ => todo!()
    }
}

fn add(dist: Option<DataSet>, src: Option<DataSet>) -> Vec<u8>{
    match (&dist, &src) {
        (match_data!(Register(_, 8, _, _)), match_data!(Register(_, 8, _, _))) =>{
            let mut ins = Instruction::new();
            ins.set_opcode(0x00);
            ins.set_reg(src.unwrap());
            ins.set_rm(dist.unwrap());
            ins.emit_machine_code()
        },
        (match_data!(Register(_, 8, _, _)), match_data!(Immediate(_, _, _))) => {
            let mut ins = Instruction::new();
            ins.set_opcode(0x80);
            ins.set_mod_rm_reg(0);
            ins.set_rm(dist.unwrap());
            ins.set_imm(src.unwrap());
            ins.emit_machine_code()
        }
        _ => todo!()
    }
}

#[test]
fn test_add1() {
    use data::Loc;
    let dist = DataSet {
        data: Data::Register(Register("bl", 8, 3, false)),
        loc: Loc::new("test", 1, 0)
    };
    let src = DataSet {
        data: Data::Register(Register("al", 8, 0, false)),
        loc: Loc::new("test", 1, 0)
    };
    for h in add(Some(dist), Some(src)) {
        eprint!("{:02x} ", h);
    }
}
#[test]
fn test_add2() { 
    use data::Loc;
    let dist = DataSet {
        data: Data::Register(Register("bl", 8, 3, false)),
        loc: Loc::new("test", 1, 0)
    };
    let src = DataSet {
        data: Data::Immediate(Immediate(10, 8, false)),
        loc: Loc::new("test", 1, 0)
    };
    for h in add(Some(dist), Some(src)) {
        eprint!("{:02x} ", h);
    }
}

pub struct Operands<'a>(pub Option<DataSet<'a>>, pub Option<DataSet<'a>>, pub Option<DataSet<'a>>, pub Option<DataSet<'a>>);

impl<'a> Operands<'a> {
    fn align(self) -> String {
        display_list(self.to_vec())
    }

    fn to_vec(self) -> Vec<DataSet<'a>>{
        let mut op = Vec::new();
        if self.0.is_some() {
            op.push(self.0.unwrap());
        }
        if self.1.is_some() {
            op.push(self.1.unwrap());
        }
        if self.2.is_some() {
            op.push(self.2.unwrap());
        }
        if self.3.is_some() {
            op.push(self.3.unwrap());
        }
        op
    }
}
fn display_list<T>(mut seq: Vec<T>) -> String 
    where T:std::fmt::Display 
{
    match seq.len() {
        0 => format!(""),
        1 => format!("{}", seq[0]),
        _ => format!("{}, {}", seq.remove(0), display_list(seq)),
    }    

}

fn db(bytes: Vec<u8>) -> String {
    format!("\tdb {}", display_list(bytes))
}

pub fn nasm(ins: &str, operands: Operands) -> String {
    match emit_mc(ins, operands) {
        Ok(ins_seq) => db(ins_seq.emit_machine_code()),
        Err(op) => format!("{} {}", ins, op.align())
    }
}
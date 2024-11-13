use crate::emit_mc;
use data::{Data, DataSet, Immediate, Memory, Register};
use tokenizer::emit_error;

use super::{Rex, ModRM, SIB};

pub struct Instruction {
    op_code: Vec<u8>,
    prefixes: Vec<u8>,
    rex: Option<Rex>,
    mod_rm: Option<ModRM>,
    sib: Option<SIB>,
    disp: Option<Immediate>,
    imm: Option<Immediate>,
}

impl Instruction {
    pub fn new() -> Self {
        Self {
            op_code: Vec::new(),
            prefixes: Vec::new(),
            rex: None,
            mod_rm: None,
            sib: None,
            disp: None,
            imm: None,
        }
    }
    #[allow(unused)]
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

    fn _set_disp(&mut self, imm: Immediate, size: usize) {
        self.disp = Some(Immediate(imm.0, size, imm.2));
    }

    fn _set_imm(&mut self, imm: Immediate, size: usize) {
        self.imm = Some(Immediate(imm.0, size, imm.2));
    }

    pub fn set_prefix(&mut self, prefix: u8) {
        self.prefixes.push(prefix);
    }

    pub fn set_opcode(&mut self, opcode: u8) {
        self.op_code.push(opcode);
    }

    fn _set_opecode_with_register(&mut self, opcode: u8, reg: Register) {
        self.set_rex_b(reg.3);
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
                self._set_disp(mem.displacement.unwrap().get_immediate().unwrap(), 8);
            }
            16 | 32 => {
                self.set_mod_rm_mod(0b10);
                self._set_disp(mem.displacement.unwrap().get_immediate().unwrap(), 32);
            }
            _ => todo!(),
        }

        // [<base> '+' <index> ('*' <scale>)*]
        if mem.index.is_some() {
            self.set_mod_rm_mod(0b100);

            let index = mem.index.unwrap();
            let scale = mem.scale.unwrap_or(1);

            self.set_sib_index(index.2);
            self.set_rex_x(index.3);
            self.set_sib_scale(scale);

            if let Some(Register(_, 64, base, b, _)) = mem.base {
                self.set_sib_base(base);
                self.set_rex_b(b);
            } else {
                self.set_sib_base(0b101);
                self.set_rex_b(false);
            }
            return;
        }

        match mem.base {
            Some(Register(_, 64, 4, _, _)) => {
                todo!()
            }
            Some(Register(_, 64, 5, b, _)) => {
                if mem.disp_size == 0 {
                    self.set_rex_b(b);
                    self.set_mod_rm_mod(0b01);
                    self._set_disp(Immediate(0, 8, false), 8);
                } else {
                    self.set_rex_b(b);
                    self.set_mod_rm_rm(5);
                }
            }
            Some(Register(_, 64, rm, b, _)) => {
                self.set_rex_b(b);
                self.set_mod_rm_rm(rm);
            }
            None => todo!(),
            _ => todo!(),
        }
    }

    pub fn set_reg(&mut self, reg: DataSet) {
        match reg.data {
            Data::Register(r) => {
                self._set_reg(r);
            }
            _ => todo!("going to be error"),
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
            _ => todo!("going to be error"),
        }
    }

    pub fn set_imm(&mut self, imm: DataSet, size: usize) {
        match imm.data {
            Data::Immediate(i) => {
                self._set_imm(i, size);
            }
            _ => {
                emit_error!(imm.location, "going to be error {:?}", imm);
            }
        }
    }
    pub fn set_disp(&mut self, imm: DataSet) {
        match imm.data {
            Data::Immediate(i) => {
                self._set_disp(i, i.1);
            }
            _ => todo!("going to be error"),
        }
    }
    pub fn set_opecode_with_register(&mut self, opcode: u8, reg: DataSet) {
        match reg.data {
            Data::Register(r) => {
                self._set_opecode_with_register(opcode, r);
            }
            _ => todo!("going to be error"),
        }
    }

    fn encode(mut self) -> Vec<u8> {
        let mut mc: Vec<u8> = Vec::new();
        mc.append(&mut self.prefixes);
        if self.rex.is_some() {
            mc.push(self.rex.unwrap().encode(true));
        }
        mc.append(&mut self.op_code);
        if self.mod_rm.is_some() {
            mc.push(self.mod_rm.unwrap().encode());
        }
        if self.sib.is_some() {
            mc.push(self.sib.unwrap().encode());
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

#[derive(Clone)]
pub struct Operands<'a>(
    pub Option<DataSet<'a>>,
    pub Option<DataSet<'a>>,
    pub Option<DataSet<'a>>,
    pub Option<DataSet<'a>>,
);

impl<'a> Operands<'a> {
    fn get_0(&self) -> Option<&DataSet> {
        if let Some(i) = &self.0 {
            Some(i)
        } else {
            None
        }
    }
    fn get_1(&self) -> Option<&DataSet> {
        if let Some(i) = &self.1 {
            Some(i)
        } else {
            None
        }
    }
    fn get_2(&self) -> Option<&DataSet> {
        if let Some(i) = &self.2 {
            Some(i)
        } else {
            None
        }
    }
    fn get_3(&self) -> Option<&DataSet> {
        if let Some(i) = &self.3 {
            Some(i)
        } else {
            None
        }
    }
}

impl<'a> std::fmt::Display for Operands<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_none() {
            write!(f, "")
        } else if self.1.is_none() {
            write!(f, "{}", self.get_0().unwrap())
        } else if self.2.is_none() {
            write!(f, "{}, {}", self.get_0().unwrap(), self.get_1().unwrap())
        } else if self.3.is_none() {
            write!(
                f,
                "{}, {}, {}",
                self.get_0().unwrap(),
                self.get_1().unwrap(),
                self.get_2().unwrap()
            )
        } else {
            write!(
                f,
                "{}, {}, {}, {}",
                self.get_0().unwrap(),
                self.get_1().unwrap(),
                self.get_2().unwrap(),
                self.get_3().unwrap()
            )
        }
    }
}

fn display_hex<T>(mut seq: Vec<T>) -> String
where
    T: std::fmt::Display + std::fmt::LowerHex,
{
    match seq.len() {
        0 => format!(""),
        1 => format!("0x{:02x}", seq[0]),
        _ => format!("0x{:02x}, {}", seq.remove(0), display_hex(seq)),
    }
}

fn db(bytes: Vec<u8>) -> String {
    format!("\tdb {}", display_hex(bytes))
}

pub fn nasm(ins: &str, operands: Operands) -> String {
    let code = format!("\n; {} {}\n", ins, operands);
    match emit_mc(ins, operands) {
        Ok(ins_seq) => code + &db(ins_seq.encode()),
        Err(op) => format!("{} {}", ins, op),
    }
    // format!("{} {}", ins, operands)
}

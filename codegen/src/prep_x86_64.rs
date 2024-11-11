use std::ops::{Index, IndexMut};

use crate::emit_mc;
use data::{Data, DataSet, Immediate, Memory, Register};
use tokenizer::emit_error;

struct Bits<const N: usize> ([bool; N]);

impl<const N: usize> Bits<N> {
    fn new(value: bool) -> Self {
        Self([value; N])
    }

    fn nth(&self, i: usize) -> u8 {
        self[i] as u8
    }
}

impl<const N: usize> Index<usize> for Bits<N> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Bits<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Clone)]
struct ModRM {
    mode: u8,
    reg: u8,
    rm: u8,
}

impl ModRM {
    fn new() -> Self {
        Self {
            mode: 0,
            reg: 0,
            rm: 0,
        }
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
        Self {
            scale: 0,
            index: 0,
            base: 0,
        }
    }

    fn set_scale(&mut self, scale: u8) {
        self.scale = match scale {
            1 => 0b00,
            2 => 0b01,
            4 => 0b10,
            8 => 0b11,
            _ => panic!("unexpected scale"),
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
        Self {
            w: false,
            r: false,
            x: false,
            b: false,
        }
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

    pub fn generate(self, flag: bool) -> u8 {
        let rex = 0b1000 * (self.w as u8)
            | 0b0100 * (self.r as u8)
            | 0b0010 * (self.x as u8)
            | 0b0001 * (self.b as u8);
        if rex != 0 || flag {
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
        Ok(ins_seq) => code + &db(ins_seq.emit_machine_code()),
        Err(op) => format!("{} {}", ins, op),
    }
    // format!("{} {}", ins, operands)
}

#[allow(dead_code)]
// look at 'https://wiki.osdev.org/X86-64_Instruction_Encoding#VEX/XOP_opcodes'
struct Vex {
    // 'inverted' extension to the MODRM.reg field
    r: bool,
    // 'inverted' extension to the SIB.index field
    x: bool,
    // 'inverted' extension to the MODRM.rm field or the SIB.base field
    b: bool,
    // specifies the opcode map to use
    map_select: Bits<5>,
    // equivalent with REX.W?
    w: bool,
    // 'inverted' additional operand for the instruction
    v: Bits<4>,
    // vector length
    l: bool,
    // Specifies an implied mandatory prefix for the opcode
    pp: Bits<2>,
}

#[allow(dead_code)]
impl Vex {
    fn new() -> Self {
        let mut map = Bits::<5>::new(false);
        map[4] = true;

        Self {
            r: true,
            x: true,
            b: true,
            map_select: map,
            w: false,
            v: Bits::<4>::new(true),
            l: false,
            pp: Bits::<2>::new(false),
        }
    }

    fn set_r(&mut self, r: bool) {
        self.r = r;
    }

    fn set_x(&mut self, x: bool) {
        self.x = x;
    }

    fn set_b(&mut self, b: bool) {
        self.b = b;
    }

    fn set_map_select(&mut self, map_select: Bits<5>) {
        self.map_select = map_select;
    }

    fn set_w(&mut self, w: bool) {
        self.w = w;
    }

    fn set_v(&mut self, v: [bool; 4]) {
        self.v = v;
    }

    fn set_l(&mut self, l: bool) {
        self.l = l;
    }

    fn set_pp(&mut self, pp: [bool; 2]) {
        self.pp = pp;
    }

    fn get_vex_prefix(&self) -> u8 {
        if self.x && self.b && !self.w && self.map_select == [false, false, false, false, true] {
            // VEX.~X == 1, VEX.~B == 1, VEX.W/E == 0 and map_select == b00001
            return 0xc5;
        } else {
            return 0xc4;
        }
    }

    fn encode(self) -> Vec<u8> {
        let mut vex: Vec<u8> = Vec::new();
        match self.get_vex_prefix() {
            0xc5 => {
                vex.push(0xc5);
                let r = (self.r as u8) << 7;
                let vvvv = {
                    (self.v[0] as u8) << 6 + (self.v[1] as u8) << 5 + (self.v[2] as u8) << 4 + (self.v[3] as u8) << 3
                };
                let l = (self.l as u8) << 2;
                let pp = {
                    (self.pp[0] as u8) << 2 + (self.pp[1] as u8)
                };
                vex.push(r | vvvv | l | pp);
            },
            0xc4 => {
                vex.push(0xc4);
                let r = (self.r as u8) << 7;
                let x = (self.x as u8) << 6;
                let b = (self.b as u8) << 5;
                let map_select = {
                    (self.map_select[0] as u8) << 4 + (self.v[1] as u8) << 3 + (self.v[2] as u8) << 2 + (self.v[3] as u8) << 1 + (self.v[4] as u8)
                };
                vex.push(r | x | b | map_select);

                let w = (self.w as u8) << 7;
                let vvvv = {
                    (self.v[0] as u8) << 6 + (self.v[1] as u8) << 5 + (self.v[2] as u8) << 4 + (self.v[3] as u8) << 3
                };
                let l = (self.l as u8) << 2;
                let pp = {
                    (self.pp[0] as u8) << 2 + (self.pp[1] as u8)
                };
                vex.push(w | vvvv | l | pp);
            },
            _ => {
                // never happen
                todo!();
            }
        }
        return vex;
    }
}

#[test]
fn encoding_test() {
    let mut new = Vex::new();
    todo!()
}

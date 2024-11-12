use super::Bits;

#[allow(dead_code)]
// look at 'https://wiki.osdev.org/X86-64_Instruction_Encoding#VEX/XOP_opcodes'
pub(crate) struct Vex {
    // 'inverted' extension to the MODRM.reg field
    r: Bits<1>,
    // 'inverted' extension to the SIB.index field
    x: Bits<1>,
    // 'inverted' extension to the MODRM.rm field or the SIB.base field
    b: Bits<1>,
    // specifies the opcode map to use
    map_select: Bits<5>,
    // equivalent with REX.W?
    w: Bits<1>,
    // 'inverted' additional operand for the instruction
    v: Bits<4>,
    // vector length
    l: Bits<1>,
    // Specifies an implied mandatory prefix for the opcode
    pp: Bits<2>,
}

#[allow(dead_code)]
impl Vex {
    fn new() -> Self {
        Self {
            r: Bits::make(1),
            x: Bits::make(1),
            b: Bits::make(1),
            map_select: Bits::<5>::make(0b00001),
            w: Bits::make(0),
            v: Bits::<4>::make(0b1111),
            l: Bits::make(0),
            pp: Bits::<2>::make(0b00),
        }
    }

    fn set_r(&mut self, r: u8) {
        self.r.set_num(r);
    }

    fn set_x(&mut self, x: u8) {
        self.x.set_num(x);
    }

    fn set_b(&mut self, b: u8) {
        self.b.set_num(b);
    }

    fn set_map_select(&mut self, map_select: u8) {
        self.map_select.set_num(map_select);
    }

    fn set_w(&mut self, w: u8) {
        self.w.set_num(w);
    }

    fn set_v(&mut self, v: u8) {
        self.v.set_num(v);
    }

    fn set_l(&mut self, l: u8) {
        self.l.set_num(l);
    }

    fn set_pp(&mut self, pp: u8) {
        self.pp.set_num(pp);
    }

    fn get_vex_prefix(&self) -> u8 {
        if self.x == 1 && self.b == 1 && self.w == 0 && self.map_select == 0b00001 {
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
                let r = self.r.value() << 7;
                let vvvv = self.v.value() << 3;
                let l = self.l.value() << 2;
                let pp = self.v.value();
                vex.push(r | vvvv | l | pp);
            },
            0xc4 => {
                vex.push(0xc4);
                let r = self.r.value() << 7;
                let x = self.x.value() << 6;
                let b = self.b.value() << 5;
                let map_select = self.map_select.value();
                vex.push(r | x | b | map_select);

                let w = self.w.value() << 7;
                let vvvv = self.v.value() << 3;
                let l = self.l.value() << 2;
                let pp = self.v.value();
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

use super::Bits;

#[derive(Clone)]
pub(crate) struct Rex {
    rex: Bits<4>,
    // +---+---+---+---+
    // | 0 | 1 | 2 | 3 |
    // +---+---+---+---+
    // | w | r | x | b |
    // +---+---+---+---+
}

impl Rex {
    pub(crate) fn new() -> Self {
        Self {
            rex: Bits::new()
        }
    }

    pub(crate) fn rex_w(&mut self, w: bool) {
        self.rex[0] = w;
    }

    // extention of the ModR/M of reg field
    pub(crate) fn rex_r(&mut self, r: bool) {
        self.rex[1] = r;
    }

    // extention of the SIB index field
    pub(crate) fn rex_x(&mut self, x: bool) {
        self.rex[2] = x;
    }

    // extention of the ModR/M r/m field, SIB base field, opcode reg field
    pub(crate) fn rex_b(&mut self, b: bool) {
        self.rex[3] = b;
    }

    pub fn generate(self, flag: bool) -> u8 {
        let rex = self.rex.value();
        if rex != 0 || flag {
            0x40 | rex
        } else {
            0
        }
    }
}
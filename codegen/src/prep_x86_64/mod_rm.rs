use super::Bits;

#[derive(Clone)]
pub(crate) struct ModRM {
    mode: Bits<2>,
    reg: Bits<3>,
    rm: Bits<3>,
}

impl ModRM {
    pub(crate) fn new() -> Self {
        Self {
            mode: Bits::new(),
            reg: Bits::new(),
            rm: Bits::new(),
        }
    }

    pub(crate) fn set_rm(&mut self, rm: u8) {
        self.rm.set_num(rm);
    }

    pub(crate) fn set_reg(&mut self, reg: u8) {
        self.reg.set_num(reg);
    }

    pub(crate) fn set_mode(&mut self, mode: u8) {
        self.mode.set_num(mode);
    }

    pub(crate) fn generate(self) -> u8 {
        self.mode.value() << 6 | self.reg.value() << 3 | self.rm.value()
    }
}
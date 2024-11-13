use super::Bits;

#[derive(Clone)]
pub(crate) struct SIB {
    scale: Bits<2>,
    index: Bits<3>,
    base: Bits<3>,
}

impl SIB {
    pub(crate) fn new() -> Self {
        Self {
            scale: Bits::new(),
            index: Bits::new(),
            base: Bits::new(),
        }
    }

    pub(crate) fn set_scale(&mut self, scale: u8) {
        self.scale.set_num(
            match scale {
                1 => 0b00,
                2 => 0b01,
                4 => 0b10,
                8 => 0b11,
                _ => panic!("unexpected scale"),
            }
        );
    }

    pub(crate) fn set_index(&mut self, index: u8) {
        self.index.set_num(index);
    }

    pub(crate) fn set_base(&mut self, base: u8) {
        self.base.set_num(base);
    }

    pub(crate) fn encode(self) -> u8 {
        self.scale.value() << 6 | self.index.value() << 3 | self.base.value()
    }
}
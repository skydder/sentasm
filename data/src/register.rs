#[derive(Clone, Copy, Debug)]
pub enum RegType {
    GP8,
    GP16,
    GP32,
    GP64,
    X87_80,
    MMX,
    XMM,
    YMM,
    SReg,
    CReg,
    DReg,
}

// second parameter represents its size, and the third represents its value, which later use in mod-rm part.
// the forth represents wheather reg is r8~r15.
#[derive(Clone, Copy, Debug)]
pub struct Register<'a>(pub &'a str, pub usize, pub u8, pub bool, pub RegType);

// todo: add other register
impl<'a> Register<'a> {
    pub fn is_reg(token: &'a str) -> bool {
        use crate::data_auto::{REG8, REG16, REG32, REG64};
        macro_rules! contains {
            ($REG:expr, $token:expr) => {
                $REG.iter().find_map(|reg| if *reg == $token {
                    Some(())
                } else {
                    None
                }).is_some()
            };
        }
        contains!(REG8, token) || contains!(REG16, token) || contains!(REG32, token) || contains!(REG64, token)
    }
    pub fn is_64(&self) -> bool {
        self.3
    }
    pub fn reg(&self) -> u8 {
        self.2
    }
    pub fn size(&self) -> usize {
        self.1
    }
}

impl<'a> std::fmt::Display for Register<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
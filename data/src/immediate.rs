macro_rules! split_bytes {
    ($bytes: expr, $nth: expr, $type: ty) => {
        (($bytes >> $nth) as $type, ($bytes - (($bytes >> $nth) << $nth)) as $type)
    };
}

#[derive(Clone, Copy, Debug)]
pub struct Immediate(pub u64, pub usize, pub bool);

impl Immediate {
    pub fn size_signed(i: u64) -> usize {
        if i < i8::MAX.try_into().unwrap() {
            8
        } else if i < i16::MAX.try_into().unwrap() {
            16
        } else if i < i32::MAX.try_into().unwrap() {
            32
        } else if i < i64::MAX.try_into().unwrap() {
            64
        } else {
            todo!()
        }
    }
    pub fn size_unsigned(i: u64) -> usize {
        println!("{}", u8::MAX);
        if i < u8::MAX.into() {
            8
        } else if i < u16::MAX.into() {
            16
        } else if i < u32::MAX.into() {
            32
        } else if i < u64::MAX.into() {
            64
        } else {
            todo!()
        }
    }

    pub fn generate(self) -> Vec<u8> {
        match self.1 {
            8 => {
                println!("{:?}", self);
                let byte = if self.2 && self.0 != 0 {
                    u8::MAX - (self.0 as u8 - 1)
                } else {
                    self.0 as u8
                };
                return vec![byte];
            },
            16 => {
                let byte = if self.2 {
                    u16::MAX - (self.0 as u16 - 1)
                } else {
                    self.0 as u16
                };
                let (high, low) =  split_bytes!(byte, 8, u8);
                return vec![low, high];
            },
            32 => {
                let byte = if self.2 {
                    u32::MAX - (self.0 as u32 - 1)
                } else {
                    self.0 as u32
                };
                let (high, low) = split_bytes!(byte, 16, u16);
                let (hh, hl) = split_bytes!(high, 8, u8);
                let (lh, ll) = split_bytes!(low, 8, u8);
                
                return vec![ll, lh, hl, hh];
            },
            64 => {
                let byte = if self.2 {
                    u64::MAX - (self.0 as u64 - 1)
                } else {
                    self.0 as u64
                };

                let (high, low) = split_bytes!(byte, 32, u32);
                let (hh, hl) = split_bytes!(high, 16, u16);
                let (lh, ll) = split_bytes!(low, 16, u16);
                let (hhh, hhl) = split_bytes!(hh, 8, u8);
                let (hlh, hll) = split_bytes!(hl, 8, u8);
                let (lhh, lhl) = split_bytes!(lh, 8, u8);
                let (llh, lll) = split_bytes!(ll, 8, u8);
                
                return vec![lll, llh, lhl, lhh, hll, hlh, hhl, hhh];
            }
            _ => {
                todo!()
            }
        }
        
    }
}

#[test]
fn test() {
    for i in Immediate(0x3c, 64, false).generate(){
        println!("{:02x}", i)
    }
}

impl std::fmt::Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.2 {
            write!(f, "-{}", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
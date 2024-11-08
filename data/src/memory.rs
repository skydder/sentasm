use crate::{Data, DataSet, Register};

#[derive(Debug, Clone)]
pub struct Memory<'a> {
    pub base: Option<Register<'a>>,
    pub displacement: Option<Box<DataSet<'a>>>,
    pub index: Option<Register<'a>>,
    pub scale: Option<u8>,
    pub size: usize,
    pub disp_size: usize,
}

impl<'a> Memory<'a> {
    pub fn new() -> Self {
        Self {
            base: None,
            displacement: None,
            index: None,
            scale: None,
            size: 0,
            disp_size: 0,
        }
    }

    pub fn set_base(&mut self, reg: Register<'a>) {
        self.base = Some(reg);
    }

    pub fn set_index(&mut self, reg: Register<'a>) {
        self.index = Some(reg);
    }

    pub fn set_scale(&mut self, scale: u8) {
        self.scale = Some(scale);
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn set_disp(&mut self, disp: DataSet<'a>) {
        self.disp_size = disp.size();
        self.displacement = Some(Box::new(disp));
    }

    pub fn check_size_of_reg(&self, size: usize) -> bool {
        if self.base.is_some() && self.base.unwrap().size() != size {
            return false;
        }
        if self.base.is_some() && self.index.unwrap().size() != size {
            return false;
        }
        true
    }
    pub fn size(&self) -> usize {
        self.size
    }
}

impl<'a> std::fmt::Display for Memory<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.size != 0 {
            match self.size {
                8 => write!(f, "byte"),
                16 => write!(f, "word"),
                32 => write!(f, "dword"),
                64 => write!(f, "qword"),
                _ => write!(f, ""), // should be error ....
            }?;
        }
        write!(f, "[")?;
        let mut count = 0;
        if let Some(base) = self.base {
            write!(f, "{}", base)?;
            count += 1;
        }

        if let Some(idx) = self.index {
            if count > 0 {
                write!(f, "+")?;
            }
            write!(f, "{}", idx)?;
            if let Some(scl) = self.scale {
                write!(f, "*{}", scl)?;
            }
        }

        if let Some(disp) = &self.displacement {
            match **disp {
                DataSet {
                    data: Data::Immediate(i),
                    location: _,
                } => {
                    if count > 0 && !i.2 {
                        write!(f, "+")?;
                    } else if i.2 {
                        write!(f, "-")?;
                    }
                    write!(f, "{}", i.0)?;
                }
                DataSet {
                    data: Data::Label(l),
                    location: _,
                } => {
                    if count > 0 {
                        write!(f, "+")?;
                    }
                    write!(f, "{}", l.0)?;
                }
                _ => todo!(),
            }
        }
        write!(f, "]")
    }
}

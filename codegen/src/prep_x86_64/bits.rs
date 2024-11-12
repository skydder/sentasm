
#[derive(Clone,Copy)]
pub struct Bits<const N: usize> ([bool; N]);

impl<const N: usize> Bits<N> {
    pub(crate) fn new() -> Self {
        Self([false; N])
    }

    pub(crate) fn make(value: u8) -> Self {
        let mut bits = Self::new();
        if N > 8 {
            panic!()
        } else {
            for i in 0..N {
                bits[i] = (value >> (7 - i)) & 1 == 1;
            }
            return bits;
        }
    } 

    pub(crate) fn nth(&self, i: usize) -> u8 {
        self[i] as u8
    }

    pub(crate) fn value(&self) -> u8 {
        if N > 8 {
            panic!()
        } else {
            let mut res = 0;
            for i in 0..N  {
                res |= self.nth(i) << (N - i - 1);
            }
            res
        }
    }
    
    pub(crate) fn set_num(&mut self, num: u8) {
        if N > 8 {
            panic!()
        } else {
            for i in 0..N {
                self.0[i] = (num >> (7 - i)) & 1 == 1;
            }
        }
    }
}

impl<const N: usize> std::cmp::PartialEq<u8> for Bits<N>  {
    fn eq(&self, other: &u8) -> bool {
        self.value() == *other
    }
}

impl<const N: usize> std::ops::Index<usize> for Bits<N> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> std::ops::IndexMut<usize> for Bits<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
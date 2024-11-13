use super::Bits;

pub(crate) struct Evex {
    mmm: Bits<3>,
    r_prime: Bits<1>,
    r: Bits<1>,
    b: Bits<1>,
    x: Bits<1>,
    pp: Bits<2>,
    vvvv: Bits<4>,
    w: Bits<1>,
    aaa: Bits<3>,
    v_prime: Bits<1>,
    b_: Bits<1>,
    l_prime_l: Bits<2>,
    z: Bits<1>,
}

impl Evex {
    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(0x62);

        // P0
        let r = self.r.value() << 7;
        let x = self.x.value() << 6;
        let b = self.b.value() << 5;
        let r_prime = self.r_prime.value() << 4;
        let mmm = self.mmm.value();
        let p0 = r | b | x | r_prime | mmm;
        bytes.push(p0);

        // P1
        let w = self.w.value() << 7;
        let vvvv = self.vvvv.value() << 3;
        let pp = self.pp.value();
        let p1 = w | vvvv | 0b100 | pp;
        bytes.push(p1);

        // P2
        let z = self.z.value() << 7;
        let l_prime_l = self.l_prime_l.value() << 5;
        let b_ = self.b_.value() << 4;
        let v_prime = self.v_prime.value() << 3;
        let aaa = self.aaa.value();
        let p2 = z | l_prime_l | b_ | v_prime | aaa;
        bytes.push(p2);

        bytes
    }
}
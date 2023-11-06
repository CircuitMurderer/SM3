use crate::func::{p_0, p_1, t_j, ff_j, gg_j};

pub struct SM3Hasher {
    iv: [u32; 8],
    pub msg: Vec<u8>,
}

impl SM3Hasher {
    pub fn new(val: &str) -> Self {
        Self {
            iv: SM3Hasher::constr_iv(),
            msg: SM3Hasher::constr_msg(val),
        }
    }

    fn constr_iv() -> [u32; 8] {
        [
            0x7380166f, 0x4914b2b9, 0x172442d7, 0xda8a0600, 
            0xa96f30bc, 0x163138aa, 0xe38dee4d, 0xb0fb0e4e,
        ]
    }

    fn constr_msg(value: &str) -> Vec<u8> {
        value.as_bytes().to_vec()
    }
    
    fn pad(&self) -> Vec<u8> {
        let mut msgv = self.msg.clone();
        let len = (msgv.len() << 3) as u64;

        msgv.push(0x80);
        for _ in 0..(56 - msgv.len() % 64) {
            msgv.push(0x00);
        }

        msgv.extend(len.to_be_bytes().to_vec());
        msgv
    }

    //fn grp(&self) -> Vec<u32> {
//
    //}

    fn block(&mut self, bi: [u8; 64]) -> ([u32; 68], [u32; 64]) {
        let mut w0 = [0u32; 68];
        let mut w1 = [0u32; 64];
        // let k = u32::f

        for j in 0..16 {
            w0[j] = u32::from_be_bytes([
                bi[j * 4], 
                bi[j * 4 + 1], 
                bi[j * 4 + 2], 
                bi[j * 4 + 3]
            ]);
        }

        for j in 16..68 {
            w0[j] = p_1(w0[j - 16] ^ w0[j - 9] ^ w0[j - 3].rotate_left(15) ^ 
                        w0[j - 13].rotate_left(7) ^ w0[j - 6]);
        }

        for j in 0..64 {
            w1[j] = w0[j] ^ w0[j + 4];
        }

        let mut a = self.iv[0];
        let mut b = self.iv[1];
        let mut c = self.iv[2];
        let mut d = self.iv[3];
        let mut e = self.iv[4];
        let mut f = self.iv[5];
        let mut g = self.iv[6];
        let mut h = self.iv[7];

        for j in 0..64 {
            let ss1 = (a
                .rotate_left(12)
                .wrapping_add(e)
                .wrapping_add(t_j(j)
                .rotate_left(j as u32)))
                .rotate_left(7);
            let ss2 = ss1 ^ a.rotate_left(12);
            let tt1 = ff_j(j, a, b, c)
                .wrapping_add(d)
                .wrapping_add(ss2)
                .wrapping_add(w1[j]);
            let tt2 = gg_j(j, e, f, g)
                .wrapping_add(h)
                .wrapping_add(ss1)
                .wrapping_add(w0[j]);

            d = c;
            c = b.rotate_left(9);
            b = a;
            a = tt1;
            h = g;
            g = f.rotate_left(19);
            f = e;
            e = p_0(tt2);   
        }

        self.iv[0] ^= a;
        self.iv[1] ^= b;
        self.iv[2] ^= c;
        self.iv[3] ^= d;
        self.iv[4] ^= e;
        self.iv[5] ^= f;
        self.iv[6] ^= g;
        self.iv[7] ^= h;

        (w0, w1)
    }

    pub fn hashed(&self) -> [u8; 32] {
        [0; 32]
    }

    pub fn hash(&self) -> String {
        String::from("OK!")
    }
}

pub fn quick_sm3_hash(s: &str) -> String {
    let hasher = SM3Hasher::new(s);
    hasher.hash()
}


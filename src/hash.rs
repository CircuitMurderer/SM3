use crate::func::{p_0, p_1, t_j, ff_j, gg_j, build_hex};


enum Regs {
    A = 0, B, C, D, E, F, G, H
}


pub struct SM3Hasher {
    iv: Vec<u32>,
    pub msg: Vec<u8>,
}


impl SM3Hasher {
    pub fn new(val: &str) -> Self {
        Self {
            iv: SM3Hasher::constr_iv(),
            msg: SM3Hasher::constr_msg(val),
        }
    }

    fn constr_iv() -> Vec<u32> {
        vec![
            0x7380166fu32, 0x4914b2b9u32, 0x172442d7u32, 0xda8a0600u32, 
            0xa96f30bcu32, 0x163138aau32, 0xe38dee4du32, 0xb0fb0e4eu32,
        ]
    }

    fn constr_msg(value: &str) -> Vec<u8> {
        value.as_bytes().to_vec()
    }
    
    fn pad(&self) -> Vec<u8> {
        let mut msgv = self.msg.clone();
        let len = (msgv.len() << 3) as u64;

        msgv.push(0x80);
        msgv.extend(vec![0x0u8; 56 - msgv.len() % 64]);
        msgv.extend(len.to_be_bytes().to_vec());

        assert_eq!(msgv.len() % 64, 0);
        msgv
    }

    fn update(&mut self, bi: &[u8]) {
        let mut w0: Vec<u32> = Vec::with_capacity(68);
        bi.chunks(4).for_each(|w| {
            w0.push(
                u32::from_be_bytes(w
                    .split_at(std::mem::size_of::<u32>())
                    .0
                    .try_into()
                    .unwrap()
                )
            );
        });

        (16usize..68).for_each(|j| {
            w0.push(p_1(w0[j - 16] ^ 
                w0[j - 9] ^ 
                w0[j - 3].rotate_left(15)) ^
                w0[j - 13].rotate_left(7) ^ 
                w0[j - 6]
            );
        });

        let mut w1 = vec![0u32; 64];
        w1 = w1.iter().enumerate().map(|(j, _)|
            w0[j] ^ w0[j + 4]
        ).collect();

        // a b c d e f g h
        // 0 1 2 3 4 5 6 7
        let mut r = self.iv.clone();

        for j in 0..64 {
            let ss1 = (r[Regs::A as usize]
                        .rotate_left(12)
                        .wrapping_add(r[Regs::E as usize])
                        .wrapping_add(t_j(j)
                        .rotate_left(j as u32)))
                        .rotate_left(7);

            let ss2 = ss1 ^ r[Regs::A as usize]
                        .rotate_left(12);

            let tt1 = ff_j(j, r[Regs::A as usize], 
                        r[Regs::B as usize], r[Regs::C as usize])
                        .wrapping_add(r[Regs::D as usize])
                        .wrapping_add(ss2)
                        .wrapping_add(w1[j]);

            let tt2 = gg_j(j, r[Regs::E as usize], 
                        r[Regs::F as usize], r[Regs::G as usize])
                        .wrapping_add(r[Regs::H as usize])
                        .wrapping_add(ss1)
                        .wrapping_add(w0[j]);

            r[Regs::D as usize] = r[Regs::C as usize];
            r[Regs::C as usize] = r[Regs::B as usize].rotate_left(9);
            r[Regs::B as usize] = r[Regs::A as usize];
            r[Regs::A as usize] = tt1;
            r[Regs::H as usize] = r[Regs::G as usize];
            r[Regs::G as usize] = r[Regs::F as usize].rotate_left(19);
            r[Regs::F as usize] = r[Regs::E as usize];
            r[Regs::E as usize] = p_0(tt2);   
        }

        self.iv = self.iv
                    .iter()
                    .enumerate()
                    .map(|(i, v)| r[i] ^ v)
                    .collect();
    }

    pub fn hash(&mut self) -> String {
        let msgv = self.pad();
        msgv.chunks(64).for_each(|blk| {
            self.update(blk);
        });
        
        let mut resv: Vec<u8> = Vec::with_capacity(32);
        self.iv.iter().for_each(|k| {
            resv.extend(k.to_be_bytes().to_vec());
        });

        assert_eq!(resv.len(), 32);
        build_hex(resv.as_slice())
    }
}


pub fn sm3_hash(s: &str) -> String {
    let mut hasher = SM3Hasher::new(s);
    let hsh_res = hasher.hash();

    println!("Text length: {}", s.len());
    println!("Plain text: {}", s);
    println!("Cipher text: {}\n", hsh_res);
    hsh_res
}

use crate::func::{p_0, p_1, t_j, ff_j, gg_j, u8_to_hex};


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
        for _ in 0..(56 - msgv.len() % 64) {
            msgv.push(0x00);
        }

        msgv.extend(len.to_be_bytes().to_vec());
        assert_eq!(msgv.len() % 64, 0);
        msgv
    }

    fn update(&mut self, bi: &[u8]) {
        let mut w0 = [0u32; 68];
        let mut w1 = [0u32; 64];

        for j in 0..16 {
            w0[j] = u32::from_be_bytes(
                (bi[(j * 4)..(j * 4 + 4)])
                    .split_at(std::mem::size_of::<u32>())
                    .0
                    .try_into()
                    .unwrap()
            )
        }

        for j in 16..68 {
            w0[j] = p_1(w0[j - 16] ^ 
                        w0[j - 9] ^ 
                        w0[j - 3].rotate_left(15)) ^
                        w0[j - 13].rotate_left(7) ^ 
                        w0[j - 6];
        }

        for j in 0..64 {
            w1[j] = w0[j] ^ w0[j + 4];
        }

        // a b c d e f g h
        // 0 1 2 3 4 5 6 7
        let mut r = self.iv.clone();

        for j in 0..64 {
            let ss1 = (r[0]
                            .rotate_left(12)
                            .wrapping_add(r[4])
                            .wrapping_add(t_j(j)
                            .rotate_left(j as u32)))
                            .rotate_left(7);
            let ss2 = ss1 ^ r[0].rotate_left(12);

            let tt1 = ff_j(j, r[0], r[1], r[2])
                            .wrapping_add(r[3])
                            .wrapping_add(ss2)
                            .wrapping_add(w1[j]);
            let tt2 = gg_j(j, r[4], r[5], r[6])
                            .wrapping_add(r[7])
                            .wrapping_add(ss1)
                            .wrapping_add(w0[j]);

            r[3] = r[2];
            r[2] = r[1].rotate_left(9);
            r[1] = r[0];
            r[0] = tt1;
            r[7] = r[6];
            r[6] = r[5].rotate_left(19);
            r[5] = r[4];
            r[4] = p_0(tt2);   
        }

        self.iv = self.iv
                    .iter()
                    .enumerate()
                    .map(|(i, v)| r[i] ^ v)
                    .collect();
    }

    pub fn hash(&mut self) -> String {
        let msgv = self.pad();

        let mut to_u: &[u8];
        let mut buf = msgv.split_at(0).1;
        while buf.len() > 0 {
            (to_u, buf) = buf.split_at(64);
            self.update(to_u);
        }
        
        let mut resv: Vec<u8> = Vec::new();
        for k in self.iv.iter() {
            resv.extend(k.to_be_bytes().to_vec());
        }

        assert_eq!(resv.len(), 32);
        u8_to_hex(&resv)
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

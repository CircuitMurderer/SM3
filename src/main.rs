use sm3::hash::sm3_hash;

fn main() {
    let hsh_res_l = sm3_hash("abcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd");
    assert_eq!("debe9ff92275b8a138604889c18e5a4d6fdb70e5387e5765293dcba39c0c5732", hsh_res_l);
    
    let hsh_res_s = sm3_hash("abc");
    assert_eq!("66c7f0f462eeedd9d1f2d46bdc10e4e24167c4875cf2f7a2297da02b8f4ba8e0", hsh_res_s);
}
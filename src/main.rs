use sm3::hash::sm3_hash;

fn main() {
    let hsh_res_l = sm3_hash("abcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd");
    assert_eq!("debe9ff92275b8a138604889c18e5a4d6fdb70e5387e5765293dcba39c0c5732", hsh_res_l);
    
    let hsh_res_s = sm3_hash("abc");
    assert_eq!("66c7f0f462eeedd9d1f2d46bdc10e4e24167c4875cf2f7a2297da02b8f4ba8e0", hsh_res_s);

    let hsh_res_a = sm3_hash("Hello World!");
    assert_eq!("0ac0a9fef0d212aa76a3c431f793853ce145659ca1d14b114e96c1215cf26582", hsh_res_a);
}
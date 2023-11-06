use sm3::hash::sm3_hash;

#[test]
fn sm3_test() {
    assert_eq!("66c7f0f462eeedd9d1f2d46bdc10e4e24167c4875cf2f7a2297da02b8f4ba8e0", sm3_hash("abc"));
}
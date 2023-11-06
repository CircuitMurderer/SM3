use sm3::sm3::quick_sm3_hash;

#[test]
fn sm3_test() {
    println!("{}", quick_sm3_hash("Hello World!"));
}
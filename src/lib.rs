pub fn test_fn() -> String {
    String::from_str("hello")
}


#[test]
fn it_works() {
    assert_eq!(test_fn().as_slice(), "hello");
}


// vim:set ai et ts=4 sw=4 sts=4 fenc=utf-8:

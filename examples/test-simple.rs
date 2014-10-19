extern crate relex;


#[cfg(not(test))]
fn main() {
    println!("relex says: {}", relex::test_fn().as_slice());
}


// vim:set ai et ts=4 sw=4 sts=4 fenc=utf-8:

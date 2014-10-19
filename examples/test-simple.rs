extern crate relex;


#[cfg(not(test))]
fn main() {
    let parse_result = relex::lex("haha", []);

    match parse_result {
        Some(thing) => (),
        None => {
            println!("Parse failed!");
        }
    };
}


// vim:set ai et ts=4 sw=4 sts=4 fenc=utf-8:

use lib_airnav::parsers::arinc424::parser::Arinc424Parser;
use std::path::Path;

#[test]
fn test_parse_file() {
    let path = Path::new("data/FAACIFP18.txt");
    let _ = Arinc424Parser::parse_file(path).unwrap();
}
// SPACEAENRT   KL09G P 0     W    N31300000W12000

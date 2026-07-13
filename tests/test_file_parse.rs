use lib_airnav::parsers::arinc424::parser::{Arinc424Parser, Arinc424Version};
use std::path::Path;

#[test]
fn test_parse_file() {
    let path = Path::new("data/FAACIFP18.txt");
    let _ = Arinc424Parser::new(Arinc424Version::Rev18FAA)
        .parse_file(path)
        .unwrap();
}

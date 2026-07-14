use lib_airnav::parsers::arinc424::parser::{Arinc424Parser, Arinc424Version};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
#[test]
fn test_parse_file() {
    let path = Path::new("data/FAACIFP18.txt");
    let file = File::open(path).unwrap();
    let parser = Arinc424Parser::new(Arinc424Version::Rev18FAA);
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        let record = parser.parse(line.as_bytes());
        match record {
            Ok(_) => {}
            Err(e) => println!("Error: {:?}", e),
        }
    }
}

# ARINC 424 [![Read the Docs](https://img.shields.io/badge/read%20the%20docs-d2991d)](https://birdhalfbaked.github.io/arinc424-rs/doc/arinc424/index.html)

This project targets proper reading of **ARINC 424** data. 

After reviewing both data, a format that will make sense across both will emerge and offer a standard representation that can be used with both
sources.

## Motivation

There are as of writing this 4 different ARINC 424 libraries. Normally I would reach out to try and work together, however it is clear many are abandoned partial-implementations with no path forward to full spec support. In most cases, they only implement records with a happy path of field validation and likely do not have access to the full specification.

What this offers is a hope for a fully-compliant set of ARINC 424 readers and perhaps later writers/multi-format (de)serializers.

Contributions to this are based on actual specification documentation available from ARINC IA (and supported by AEEC, AMC, and FSEMC) via SAE International. 

## ARINC 424 Support

### Revisions supported

* ARINC 424-18
* ARINC 424-18 (with FAA extensions)
    * Note: This is bespoke and the exceptions are noted by the FAA in each CIFP dump (see below)
* ARINC 424-23

### Data Reading and definitions

Line deserialization and definitions that support this follow the revisions supported.

All fields are manually curated and defined as best as possible to capture the intent of the specification. There are errata and ambiguous interpretations of data, thus not all fields will be perfect on first use. Please raise issues as needed.

As of now all of the records under the scope of the specification are defined and should work, though I am actively working on getting dumps from major providers which would allow me to verify correctness.

### Testing with FAA CIFP dump

There is a data file available from the FAA at the following url: https://www.faa.gov/air_traffic/flight_info/aeronav/digital_products/cifp/download/

I have tested with this, but issues are sure to arise as this is a young project. Please raise issues as needed.
Note that there are FAA-specific exceptions to how the standard is used, and thus it is important to read the accompanying documentation for each dump (`CIFP Readme {cycle}.pdf`).

You can modify the test file location in `tests/test_file_parse.rs` and run the tests to verify the parser works.

### Usage notes

### Memory and lifetimes

The ARINC 424 parser is designed for efficient memory use: parsed records borrow from the caller's input buffer rather than copying field data into owned structures. **The caller is responsible for keeping source data alive for as long as parsed records are used, and for copying record data into owned types when longer retention is needed.**

Example:

```rust
use lib_airnav::parsers::arinc424::parser::{Arinc424Parser, Arinc424Version};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    // open the file
    let path = Path::new("data/FAACIFP18.txt");
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    
    // create an FAA parser
    let parser = Arinc424Parser::new(Arinc424Version::Rev18FAA);

    // read the file line by line
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            }
        };
        if line.len() == 0 {
            continue;
        }
        if let Ok(record) = parser.parse(line.as_bytes()){
            // do stuff
        } else {
            println!("Error: {:?}", record.err());
        }
    }
}

```
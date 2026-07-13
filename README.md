# Lib-airnav [![Read the Docs](https://img.shields.io/badge/read%20the%20docs-d2991d)](https://birdhalfbaked.github.io/airnav-rs/doc/lib_airnav/index.html)

This project helps process air navigation data in a structured way. It targets **ARINC 424** today and is intended to grow toward **AIXM** sources as well.

After reviewing both data, a format that will make sense across both will emerge and offer a standard representation that can be used with both
sources.

## Goals

- Bridge parsing/verification/export of the two standards present for FMS data:
    - AIXM
    - ARINC + extra XML
- Enable a standardized representation that allows users to quickly translate data to their needs within Rust applications
- Fix some of the issues with representation of data using a higher-level layer that can feed data from multiple sources

## ARINC 424

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

## AIXM

Next step after ARINC 424 is done, though should be easier as the schema is well defined and fits within XML parsing semantics nicely already
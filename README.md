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

Parsing and definitions follow **ARINC 424-23** (Specification 424, Change 23).

All fields are manually curated and defined as best as possible to capture the intent of the specification. There are errata and ambiguous interpretations of data, thus not all fields will be perfect on first use. Please raise issues as needed.


## AIXM

Next step after ARINC 424 is done, though should be easier as the schema is well defined and fits within XML parsing semantics nicely already
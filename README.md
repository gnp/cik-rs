cik
====
A `CIK` type for working with validated (syntax only)
[Central Index Keys (CIKs)](https://www.sec.gov/page/edgar-glossary#cik) as defined in the
[U.S. Securities and Exchange Commission's (SEC)](https://www.sec.gov)
[Electronic Data Gathering, Analysis, and Retrieval system (EDGAR)](https://www.sec.gov/edgar.shtml) documentation.

A CIK is a number of up to 10 digits length. They are sometimes rendered as strings with or without leading zeros, and
sometimes are represented as integers of a sufficient number of bits to represent a 10-digit number (typically 64 bits
because the maximum CIK value is 9,999,999,999 and the maximum value of a signed 32-bit integer is only 2,147,483,64;
and the maximum value of an unsigned 32-bit integer is still to low at 4,294,967,295).

As of 2022-05-29 the "Company Facts" data set's minimum CIK value is 1,750 and the maximum is 1,923,807. See
the "Bulk data" section on the SEC's
[EDGAR Application Programming Interfaces](https://www.sec.gov/edgar/sec-api-documentation) page for more information on
this data set. Such values fit comfortably in 7 decimal digits or 32 bits (signed or unsigned), so you might encounter
CIKs stored in values of less than 64 bits.

Nonetheless, this library uses 64-bit values to ensure full conformance with the CIK definition of up to 10 decimal
digits.

This crate is part of the Financial Identifiers series:

* [CIK](https://crates.io/crates/cik): Central Index Key (SEC EDGAR)
* [CUSIP](https://crates.io/crates/cusip): Committee on Uniform Security Identification Procedures (ANSI X9.6-2020)
* [ISIN](https://crates.io/crates/isin): International Securities Identification Number (ISO 6166:2021)
* [LEI](https://crates.io/crates/lei): Legal Entity Identifier (ISO 17442:2020)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cik = "0.1"
```


## Example

```rust
use cik;
fn main() -> () {
    let cik_string = "320193";
    match cik::parse(cik_string) {
        Ok(cik) => {
            println!("Parsed CIK: {}", cik.to_string()); // "320193"
            println!("  Integer value: {}", cik.value()); // 320193
        }
        Err(err) => panic!("Unable to parse CIK {}: {}", cik_string, err),
    }
}
```


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

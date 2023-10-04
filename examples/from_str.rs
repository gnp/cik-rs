use cik::CIK;
use std::str::FromStr;

fn main() {
    let cik_string = "320193";
    match CIK::from_str(cik_string) {
        Ok(cik) => {
            println!("Parsed CIK: {}", cik.to_string()); // "320193"
            println!("  Integer value: {}", cik.value()); // 320193
        }
        Err(err) => panic!("Unable to convert {} to a CIK: {}", cik_string, err),
    }
}

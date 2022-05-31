use cik::CIK;

fn main() -> () {
    let cik_value = 320193;
    match CIK::try_from(cik_value) {
        Ok(cik) => {
            println!("Parsed CIK: {}", cik.to_string()); // "320193"
            println!("  Integer value: {}", cik.value()); // 320193
        }
        Err(err) => panic!("Unable to convert {} to a CIK: {}", cik_value, err),
    }
}

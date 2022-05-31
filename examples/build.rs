use cik;

fn main() -> () {
    let cik_value = 320193;
    match cik::build(cik_value) {
        Ok(cik) => {
            println!("Parsed CIK: {}", cik.to_string()); // "320193"
            println!("  Integer value: {}", cik.value()); // 320193
        }
        Err(err) => panic!("Unable to build CIK from {}: {}", cik_value, err),
    }
}

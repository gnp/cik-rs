fn main() {
    let cik_string = "320193";
    match cik::parse(cik_string) {
        Ok(cik) => {
            println!("Parsed CIK: {}", cik.to_string()); // "320193"
            println!("  Integer value: {}", cik.value()); // 320193
        }
        Err(err) => panic!("Unable to parse CIK from {}: {}", cik_string, err),
    }
}

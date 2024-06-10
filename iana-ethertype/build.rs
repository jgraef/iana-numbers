use iana_build_tools::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {

}

fn main() -> Result<(), Error> {
    let mut writer = iana_build_tools::out_file("generated.rs");
    let records = iana_build_tools::parse::<Record>("ieee-802-numbers-1.csv");
    Ok(())
}
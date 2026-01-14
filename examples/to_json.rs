#[cfg(feature = "serde")]
use cdf::{cdf::Cdf, error::CdfError};
#[cfg(feature = "serde")]
use std::{fs::File, io::BufReader, io::Write, path::PathBuf};

#[cfg(feature = "serde")]
fn to_json_from_json(filename: &str) -> Result<(), CdfError> {
    let input_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "data", filename]
        .iter()
        .collect();

    let cdf = Cdf::read_cdf_file(input_file)?;

    let j = serde_json::to_string_pretty(&cdf).map_err(|err| CdfError::Other(err.to_string()))?;

    let output_file: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "examples",
        "data",
        filename.replace(".cdf", ".json").as_str(),
    ]
    .iter()
    .collect();

    let mut buffer = File::create(output_file.clone())?;
    write!(buffer, "{}", j)?;

    let f = File::open(output_file)?;
    let reader = BufReader::new(f);
    let cdf_read: Cdf =
        serde_json::from_reader(reader).map_err(|err| CdfError::Other(err.to_string()))?;
    // dbg!(cdf_read.cdr);
    Ok(())
}

fn main() {
    // `ulysses.cdf` is a pretty large file. Uncomment to convert this to json.

    // #[cfg(feature = "serde")]
    // to_json_from_json("ulysses.cdf").unwrap();

    #[cfg(feature = "serde")]
    to_json_from_json("test_alltypes.cdf").unwrap();
}

use cdf::cdf::Cdf;
use std::path::PathBuf;

fn read_cdf(filename: &str) {
    let input_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "data", filename]
        .iter()
        .collect();

    let _ = Cdf::read_cdf_file(input_file).unwrap();
}

fn main() {
    // read_cdf("ulysses.cdf").unwrap();

    read_cdf("test_alltypes.cdf");
    read_cdf("ulysses.cdf");
}

use std::{
    fs::File,
    io::BufWriter,
    path::{
        Path,
        PathBuf,
    },
};

pub use color_eyre::eyre::{
    bail,
    eyre as error,
    Error,
};
pub use csv;
pub use serde;
use serde::Deserialize;

pub mod phf {
    pub use phf;
    pub use phf_codegen;
    pub use phf_shared;
}

pub fn init() -> Result<(), Error> {
    color_eyre::install()?;
    Ok(())
}

pub fn parse<T: for<'de> Deserialize<'de>>(file: impl AsRef<Path>) -> impl Iterator<Item = T> {
    let file = file.as_ref();
    println!("cargo::rerun-if-changed={}", file.display());
    let reader = csv::Reader::from_path(file).expect("Could not open file");
    reader
        .into_deserialize()
        .map(|result| result.expect("Could not parse record"))
}

pub fn out_file(file_name: impl AsRef<Path>) -> BufWriter<File> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    let out_file = out_dir.join(file_name);
    let file = File::create(&out_file)
        .unwrap_or_else(|_| panic!("Could not open file: {}", out_file.display()));
    BufWriter::new(file)
}

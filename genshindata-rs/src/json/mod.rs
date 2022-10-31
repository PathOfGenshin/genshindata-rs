use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
};

#[derive(Debug)]
pub enum JsonError {
    FileOpen(io::ErrorKind),
    Deserialization(serde_json::Error),
}

pub fn load_json<T: serde::de::DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, JsonError> {
    let file = File::open(path).map_err(|file_err| JsonError::FileOpen(file_err.kind()))?;
    let reader = BufReader::new(file);

    let deserialized = serde_json::from_reader(reader).map_err(JsonError::Deserialization)?;

    Ok(deserialized)
}

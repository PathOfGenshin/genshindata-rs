use std::{fs, io, path::Path};

#[derive(Debug)]
pub enum JsonError {
    FileOpen(io::ErrorKind),
    Deserialization,
}

pub fn load_json<T: serde::de::DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, JsonError> {
    let mut file_bytes = fs::read(path).map_err(|read_err| JsonError::FileOpen(read_err.kind()))?;
    let deserialized =
        simd_json::serde::from_slice(&mut file_bytes).map_err(|_| JsonError::Deserialization)?;
    Ok(deserialized)
}

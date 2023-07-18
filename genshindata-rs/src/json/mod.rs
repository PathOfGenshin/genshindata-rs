use std::{
    env, fmt, fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum JsonError {
    EnvReadError(String),
    FileOpen(io::ErrorKind),
    Deserialization(String),
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EnvReadError(env_var) => write!(
                f,
                "failed to read JSON file due to missing environment variable {}",
                env_var
            ),
            Self::FileOpen(io_err) => {
                write!(f, "failed to open JSON file - {}", io_err)
            }
            Self::Deserialization(msg) => write!(f, "failed to deserialize JSON file - {}", msg),
        }
    }
}

const GAME_DATA_PATH_ENV_KEY: &str = "GAME_DATA_PATH";

pub fn get_game_resources_path() -> Result<String, JsonError> {
    env::var(GAME_DATA_PATH_ENV_KEY)
        .map_err(|_| JsonError::EnvReadError(GAME_DATA_PATH_ENV_KEY.into()))
}

pub fn read_excelbinoutput(json_file: &str) -> Result<PathBuf, JsonError> {
    let game_resources_path = get_game_resources_path()?;
    let path: PathBuf = [&game_resources_path, "ExcelBinOutput", json_file]
        .iter()
        .collect();
    Ok(path)
}

pub fn read_textmap(textmap_file: &str) -> Result<PathBuf, JsonError> {
    let game_resources_path = get_game_resources_path()?;
    let path: PathBuf = [&game_resources_path, "TextMap", textmap_file]
        .iter()
        .collect();
    Ok(path)
}

pub fn load_json<T: serde::de::DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, JsonError> {
    let mut file_bytes = fs::read(path).map_err(|read_err| JsonError::FileOpen(read_err.kind()))?;
    let deserialized = simd_json::serde::from_slice(&mut file_bytes)
        .map_err(|err| JsonError::Deserialization(err.to_string()))?;
    Ok(deserialized)
}

/// Helper function to load ExcelBinOutput JSON files via serde
pub fn load_excelbinoutput<T: serde::de::DeserializeOwned>(
    json_file: &str,
) -> Result<T, JsonError> {
    load_json::<T, PathBuf>(read_excelbinoutput(json_file)?)
}

/// Helper function to load TextMap JSON files via serde
pub fn load_textmap<T: serde::de::DeserializeOwned>(textmap_file: &str) -> Result<T, JsonError> {
    load_json::<T, PathBuf>(read_textmap(textmap_file)?)
}

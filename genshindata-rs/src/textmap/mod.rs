use std::{collections::HashMap, path::PathBuf};

use strum::{Display, EnumCount, EnumIter, EnumString};

use crate::json::{load_json, JsonError};

#[derive(Debug, Display, EnumCount, EnumIter, EnumString)]
pub enum Language {
    TextMapCHS,
    TextMapCHT,
    TextMapDE,
    TextMapEN,
    TextMapES,
    TextMapFR,
    TextMapID,
    TextMapJP,
    TextMapKR,
    TextMapPT,
    TextMapRU,
    TextMapTH,
    TextMapVI,
}

pub fn load_textmap(language: Language) -> Result<HashMap<i64, String>, JsonError> {
    let language_json_file = format!("{}.json", language);
    let path: PathBuf = ["GenshinData", "TextMap", language_json_file.as_str()]
        .iter()
        .collect();
    load_json(path)
}

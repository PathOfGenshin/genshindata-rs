use std::{collections::HashMap, path::PathBuf};

use strum::IntoEnumIterator;

use crate::{
    json::{load_json, JsonError},
    language::Language,
};

pub struct TextMap {
    pub language: Language,
    data: HashMap<i64, String>,
}

impl TextMap {
    pub fn load(language: Language) -> Result<Self, JsonError> {
        let language_file = format!("TextMap{}.json", language);
        let path: PathBuf = ["GenshinData", "TextMap", language_file.as_str()]
            .iter()
            .collect();
        let data = load_json(path)?;
        println!("Loaded {} TextMap!", language);
        Ok(Self { data, language })
    }

    pub fn get(&self, key: impl Into<i64>) -> Option<String> {
        let key: i64 = key.into();
        self.data.get(&key).cloned()
    }
}

pub struct AllTextMaps(HashMap<Language, TextMap>);

impl AllTextMaps {
    pub fn load_all() -> Self {
        Self(HashMap::from_iter(Language::iter().map(|lang| {
            (
                lang,
                TextMap::load(lang)
                    .unwrap_or_else(|_| panic!("Failed to load TextMap{}.json!", lang)),
            )
        })))
    }

    pub fn get(&self, language: Language) -> &TextMap {
        self.0
            .get(&language)
            .unwrap_or_else(|| panic!("Missing TextMap language {}", language))
    }
}

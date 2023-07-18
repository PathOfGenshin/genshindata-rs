use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::{json::load_textmap, json::JsonError, language::Language};

pub struct TextMap {
    pub language: Language,
    data: HashMap<i64, String>,
}

impl TextMap {
    pub fn load(language: Language) -> Result<Self, JsonError> {
        let textmap_file = format!("TextMap{}.json", language);
        let data = load_textmap(&textmap_file)?;
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
                TextMap::load(lang).unwrap_or_else(|e| {
                    panic!("Failed to load TextMap{}.json! Error: {}", lang, e)
                }),
            )
        })))
    }

    pub fn get(&self, language: Language) -> &TextMap {
        self.0
            .get(&language)
            .unwrap_or_else(|| panic!("Missing TextMap language {}", language))
    }
}

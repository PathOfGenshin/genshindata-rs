use std::collections::HashMap;

use genshindata_rs::language::Language;
use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
pub struct Translation(HashMap<String, String>);

#[derive(Debug, Clone)]
pub struct Translations {
    translation_by_language: HashMap<Language, Translation>,
}

impl Default for Translations {
    fn default() -> Self {
        Self {
            translation_by_language: HashMap::from_iter(
                Language::iter().map(|lang| (lang, Translation(HashMap::new()))),
            ),
        }
    }
}

impl Translations {
    pub fn put(&mut self, language: Language, key: String, value: String) {
        if let Some(translation) = self.translation_by_language.get_mut(&language) {
            translation.0.insert(key, value);
        }
    }

    pub fn get(&self, language: Language, key: String) -> Option<String> {
        if let Some(translation) = self.translation_by_language.get(&language) {
            translation.0.get(&key).cloned()
        } else {
            None
        }
    }

    pub fn get_pack(&self, language: Language) -> Translation {
        self.translation_by_language
            .get(&language)
            .unwrap_or_else(|| panic!("Failed to get translation pack for {}", language))
            .clone()
    }
}

#[derive(Debug, Clone)]
pub struct TranslatableData<T> {
    pub data: T,
    pub translations: Translations,
}

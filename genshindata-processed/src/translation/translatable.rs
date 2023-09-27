use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use strum::IntoEnumIterator;

use genshindata_rs::language::Language;

/// Translation key
#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct TKey(String);

/// Flat hash map of translation keys to translation values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationPack(HashMap<TKey, String>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllTranslations(HashMap<Language, TranslationPack>);

#[derive(Debug, Clone)]
pub struct Translated<T> {
    pub data: T,
    pub translations: AllTranslations,
}

impl AllTranslations {
    pub fn new() -> Self {
        Self(HashMap::from_iter(
            Language::iter().map(|lang| (lang, TranslationPack(HashMap::new()))),
        ))
    }

    pub fn put(&mut self, language: Language, key: &String, value: &String) {
        if let Some(translation) = self.0.get_mut(&language) {
            translation.0.insert(TKey(key.clone()), value.clone());
        }
    }

    pub fn get(&self, language: Language, key: &String) -> Option<String> {
        match self.0.get(&language) {
            Some(translation) => translation.0.get(&TKey(key.clone())).cloned(),
            None => None,
        }
    }

    pub fn get_pack(&self, language: Language) -> TranslationPack {
        self.0
            .get(&language)
            .unwrap_or_else(|| panic!("Failed to get translation pack for {}", language))
            .clone()
    }
}

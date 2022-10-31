use genshindata_rs::textmap::AllTextMaps;

use super::translatable::{TranslatableData, Translations};

pub trait Processable<T> {
    fn process(&self, all_textmaps: &AllTextMaps) -> TranslatableData<T>;
}

pub trait Translatable<T> {
    fn translations(&self, data: &T, all_textmaps: &AllTextMaps) -> Translations;
}

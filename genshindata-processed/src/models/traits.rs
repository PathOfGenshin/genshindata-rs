use crate::translation::translatable::{AllTranslations, Translated};
use genshindata_rs::textmap::AllTextMaps;

pub trait Processable<T> {
    fn process(&self, all_textmaps: &AllTextMaps) -> Translated<T>;
}

pub trait Translatable<T> {
    fn translations(&self, data: &T, all_textmaps: &AllTextMaps) -> AllTranslations;
}

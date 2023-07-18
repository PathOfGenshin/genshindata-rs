/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type KeywordEasterEggExcelConfigData = Vec<KeywordEasterEggExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct KeywordEasterEggExcelConfigDatum {
    #[serde(rename = "configID")]
    pub config_id: i64,
    pub kojhmfemegk: Vec<Kojhmfemegk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Kojhmfemegk {
    pub pnppooliabg: Pnppooliabg,
    pub deopmmncgem: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Pnppooliabg {
    #[serde(rename = "LANGUAGE_DE")]
    LanguageDe,
    #[serde(rename = "LANGUAGE_EN")]
    LanguageEn,
    #[serde(rename = "LANGUAGE_ES")]
    LanguageEs,
    #[serde(rename = "LANGUAGE_FR")]
    LanguageFr,
    #[serde(rename = "LANGUAGE_ID")]
    LanguageId,
    #[serde(rename = "LANGUAGE_IT")]
    LanguageIt,
    #[serde(rename = "LANGUAGE_JP")]
    LanguageJp,
    #[serde(rename = "LANGUAGE_KR")]
    LanguageKr,
    #[serde(rename = "LANGUAGE_PT")]
    LanguagePt,
    #[serde(rename = "LANGUAGE_RU")]
    LanguageRu,
    #[serde(rename = "LANGUAGE_SC")]
    LanguageSc,
    #[serde(rename = "LANGUAGE_TC")]
    LanguageTc,
    #[serde(rename = "LANGUAGE_TH")]
    LanguageTh,
    #[serde(rename = "LANGUAGE_TR")]
    LanguageTr,
    #[serde(rename = "LANGUAGE_VN")]
    LanguageVn,
}

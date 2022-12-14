// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type KeywordEasterEggExcelConfigData = Vec<KeywordEasterEggExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeywordEasterEggExcelConfigDatum {
    #[serde(rename = "configID")]
    pub config_id: i64,

    #[serde(rename = "FFBNEKGPLEK")]
    pub ffbnekgplek: Vec<Ffbnekgplek>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ffbnekgplek {
    #[serde(rename = "PLPFPKEKACG")]
    pub plpfpkekacg: Plpfpkekacg,

    #[serde(rename = "NLEDLOOBLKJ")]
    pub nledlooblkj: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Plpfpkekacg {
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

pub fn load() -> Result<KeywordEasterEggExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "KeywordEasterEggExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AvatarRenameExcelConfigData = Vec<AvatarRenameExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AvatarRenameExcelConfigDatum {
    pub imnelohfpop: i64,
    #[serde(rename = "type")]
    pub avatar_rename_excel_config_datum_type: String,
    pub jcnkcjdlofm: i64,
    pub napcdlbmhap: i64,
    pub nlabgckebeo: i64,
    pub lbladkcmfmn: i64,
    pub fojklabgaad: Vec<Fojklabgaad>,
    pub onmobmeecpi: String,
    pub jnpjjhfalkg: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Fojklabgaad {
    pub ipdjjpbhjbk: String,
    pub janfgpmieha: String,
}

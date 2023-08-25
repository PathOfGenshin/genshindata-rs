/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ExhibitionListExcelConfigData = Vec<ExhibitionListExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ExhibitionListExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub bkpoeldegld: i64,
    pub nfhpnideipc: i64,
    pub impcjafcgjj: i64,
    pub cpkbceibkfc: i64,
    #[serde(rename = "displayType")]
    pub display_type: DisplayType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DisplayType {
    #[serde(rename = "EXHIBITION_DISPLAY_TYPE_INT")]
    ExhibitionDisplayTypeInt,
    #[serde(rename = "EXHIBITION_DISPLAY_TYPE_INT_CHARACTER")]
    ExhibitionDisplayTypeIntCharacter,
    #[serde(rename = "EXHIBITION_DISPLAY_TYPE_TIME_MINSEC")]
    ExhibitionDisplayTypeTimeMinsec,
}

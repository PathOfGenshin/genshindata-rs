/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type OfferingLumenStoneExcelConfigData = Vec<OfferingLumenStoneExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfferingLumenStoneExcelConfigDatum {
    pub config_id: i64,
    #[serde(rename = "HJJGNBJOODF")]
    pub hjjgnbjoodf: i64,
    #[serde(rename = "HJKKCMKNDCM")]
    pub hjkkcmkndcm: i64,
    pub icon_path: IconPath,
    pub level: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IconPath {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "UI_ItemIcon_220048_02")]
    UiItemIcon220048_02,
    #[serde(rename = "UI_ItemIcon_220048_03")]
    UiItemIcon220048_03,
}

/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type WidgetUseableExcelConfigData = Vec<WidgetUseableExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WidgetUseableExcelConfigDatum {
    #[serde(rename = "materialID")]
    pub material_id: i64,
    pub can_use_in_other_world: Option<bool>,
    #[serde(rename = "ECMPLLDAMJI")]
    pub ecmplldamji: Vec<i64>,
    #[serde(rename = "DCHNLEHGMMI")]
    pub dchnlehgmmi: Option<bool>,
    pub forbidden_dungeon_list: Vec<Option<serde_json::Value>>,
    #[serde(rename = "CJJKNOIELFL")]
    pub cjjknoielfl: Vec<Option<serde_json::Value>>,
    #[serde(rename = "EPKFLBBGLBG")]
    pub epkflbbglbg: Vec<Option<serde_json::Value>>,
    #[serde(rename = "DECBDMLCAKK")]
    pub decbdmlcakk: Option<bool>,
    pub can_use_in_dungeon: Option<bool>,
    pub can_use_in_homeworld: Option<bool>,
    pub can_use_in_room: Option<bool>,
    pub can_use_in_limit_region: Option<bool>,
    #[serde(rename = "AGIHOBIIGKH")]
    pub agihobiigkh: Option<String>,
}

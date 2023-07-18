/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgTalkDetailIconExcelConfigData = Vec<GcgTalkDetailIconExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcgTalkDetailIconExcelConfigDatum {
    pub id: i64,
    pub icon_name: String,
    #[serde(rename = "type")]
    pub gcg_talk_detail_icon_excel_config_datum_type: Option<Type>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Monster,
    #[serde(rename = "NPC")]
    Npc,
}

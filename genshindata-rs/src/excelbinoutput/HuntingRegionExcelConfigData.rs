/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type HuntingRegionExcelConfigData = Vec<HuntingRegionExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HuntingRegionExcelConfigDatum {
    pub id: i64,
    pub center_pos_list: Vec<f64>,
    pub center_radius: i64,
    pub safe_clue_group: Vec<i64>,
    pub clue_group: Vec<i64>,
    pub safe_destination_group: Vec<i64>,
    pub destination_group: Vec<i64>,
    pub region_info_text_map_hash: i64,
    #[serde(rename = "PFALBAFLKJN")]
    pub pfalbaflkjn: Vec<i64>,
    #[serde(rename = "PDEGIOIPIMO")]
    pub pdegioipimo: Option<i64>,
}

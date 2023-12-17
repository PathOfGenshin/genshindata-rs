/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LanV3BoatExcelConfigData = Vec<LanV3BoatExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LanV3BoatExcelConfigDatum {
    #[serde(rename = "mapId")]
    pub map_id: i64,
    #[serde(rename = "type")]
    pub lan_v3_boat_excel_config_datum_type: String,
    #[serde(rename = "galleryId")]
    pub gallery_id: i64,
    #[serde(rename = "controlGroupId")]
    pub control_group_id: i64,
    #[serde(rename = "prepareTime")]
    pub prepare_time: i64,
    pub mldefollkjj: i64,
    pub fnlkfifdima: i64,
    pub gdeahjakpop: i64,
    pub lpfnmhlfpmn: i64,
    pub igjjcnpcpac: f64,
    #[serde(rename = "limitRegion")]
    pub limit_region: String,
    pub hbppncojelf: Vec<i64>,
    pub eapmeoefjfn: Vec<i64>,
    pub fehciolfcci: Vec<f64>,
    pub bmogcdggdlo: Option<i64>,
}

/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MapTagDataConfigData = Vec<MapTagDataConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapTagDataConfigDatum {
    pub id: i64,
    #[serde(rename = "type")]
    pub map_tag_data_config_datum_type: Option<String>,
    pub name_text_map_hash: i64,
    #[serde(rename = "CGOGMJIKAAG")]
    pub cgogmjikaag: i64,
    #[serde(rename = "DIKBJNCBKLF")]
    pub dikbjncbklf: i64,
    pub icon: String,
    pub hide_before_unlock: Option<bool>,
    pub scene_id_list: Vec<i64>,
    #[serde(rename = "OHBGPJAJBCC")]
    pub ohbgpjajbcc: Option<f64>,
    #[serde(rename = "CAIAPFHPFKK")]
    pub caiapfhpfkk: Option<f64>,
    pub unlock_by_default: Option<bool>,
    #[serde(rename = "cityID")]
    pub city_id: Option<i64>,
    #[serde(rename = "NGENKJKJEIN")]
    pub ngenkjkjein: Option<i64>,
}

/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type Ps5GroupExcelConfigData = Vec<Ps5GroupExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ps5GroupExcelConfigDatum {
    pub id: i64,
    pub name_text_map_hash: i64,
    pub icon_path: String,
    #[serde(rename = "IBOOHHBGJAH")]
    pub iboohhbgjah: String,
}

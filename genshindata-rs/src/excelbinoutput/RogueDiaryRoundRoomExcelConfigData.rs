/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type RogueDiaryRoundRoomExcelConfigData = Vec<RogueDiaryRoundRoomExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RogueDiaryRoundRoomExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub mgapgmfedeo: Option<i64>,
    pub ceckcogfkki: Vec<HashMap<String, Vec<i64>>>,
}

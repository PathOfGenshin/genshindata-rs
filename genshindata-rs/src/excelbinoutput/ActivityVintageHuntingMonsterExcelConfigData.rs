/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityVintageHuntingMonsterExcelConfigData = Vec<ActivityVintageHuntingMonsterExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityVintageHuntingMonsterExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "MJNJDOAOGDP")]
    pub mjnjdoaogdp: i64,
    pub monster_id_list: Vec<i64>,
}

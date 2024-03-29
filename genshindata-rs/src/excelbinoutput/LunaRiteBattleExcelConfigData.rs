/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LunaRiteBattleExcelConfigData = Vec<LunaRiteBattleExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LunaRiteBattleExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,
    pub region_type: RegionType,
    #[serde(rename = "consecrateID")]
    pub consecrate_id: Option<i64>,
    pub group_bundle_id: i64,
    #[serde(rename = "rewardID")]
    pub reward_id: i64,
    pub challenge_icon: String,
    pub monster_info: String,
    pub elite_monster_info: String,
    pub ruler_text_map_hash: i64,
    pub recipe_source_text_map_hash: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegionType {
    #[serde(rename = "LUNA_RITE_REGION_TYPE_DRAGONSPINE")]
    LunaRiteRegionTypeDragonspine,
    #[serde(rename = "LUNA_RITE_REGION_TYPE_LIYUE")]
    LunaRiteRegionTypeLiyue,
    #[serde(rename = "LUNA_RITE_REGION_TYPE_MENGDE")]
    LunaRiteRegionTypeMengde,
}

/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FungusTrainingDungeonExcelConfigData = Vec<FungusTrainingDungeonExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FungusTrainingDungeonExcelConfigDatum {
    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,
    pub mjhabfdclem: i64,
    pub nchjijhoggn: Nchjijhoggn,
    #[serde(rename = "unlockDay")]
    pub unlock_day: i64,
    pub gdfnommcmca: i64,
    #[serde(rename = "galleryId")]
    pub gallery_id: i64,
    pub lpblofahang: Vec<i64>,
    pub klniohkfbpc: Vec<i64>,
    pub kafchaijmed: i64,
    pub jibnapiomjj: i64,
    pub ppdfekoomag: Vec<i64>,
    #[serde(rename = "baseScore")]
    pub base_score: i64,
    pub lhomiaaahcl: Option<i64>,
    pub jpihddhhejg: Option<i64>,
    pub olapehikffl: i64,
    #[serde(rename = "watcherIds")]
    pub watcher_ids: Vec<i64>,
    pub fekpcenhoda: i64,
    pub hfleoalanho: i64,
    pub llfgeabnalm: i64,
    pub njaihakaabg: i64,
    pub ojfbcgdhejj: Vec<i64>,
    pub bmanhhlggof: Option<i64>,
    pub agjmghnlmii: Option<i64>,
    pub alggmogfmog: Option<i64>,
    pub onbjebgmlib: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Nchjijhoggn {
    #[serde(rename = "FUNGUS_TRAINING_DUNGEON_ATTACK")]
    FungusTrainingDungeonAttack,
    #[serde(rename = "FUNGUS_TRAINING_DUNGEON_DEFEND")]
    FungusTrainingDungeonDefend,
}

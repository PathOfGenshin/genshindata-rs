/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type UgcDungeonScoreExcelConfigData = Vec<UgcDungeonScoreExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UgcDungeonScoreExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "type")]
    pub ugc_dungeon_score_excel_config_datum_type: Type,
    #[serde(rename = "KPGNAELBFEH")]
    pub kpgnaelbfeh: String,
    pub score: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename = "UGC_DUNGEON_SCORE_CHEST_OPEN")]
    UgcDungeonScoreChestOpen,
    #[serde(rename = "UGC_DUNGEON_SCORE_COIN")]
    UgcDungeonScoreCoin,
    #[serde(rename = "UGC_DUNGEON_SCORE_MONSTER_DIE")]
    UgcDungeonScoreMonsterDie,
    #[serde(rename = "UGC_DUNGEON_SCORE_ROOM_SETTLE")]
    UgcDungeonScoreRoomSettle,
}

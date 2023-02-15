// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MonsterRelationshipExcelConfigData = Vec<MonsterRelationshipExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterRelationshipExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "tagStr")]
    pub tag_str: String,

    #[serde(rename = "monsterRarity")]
    pub monster_rarity: MonsterRarity,

    #[serde(rename = "HEHBBIIDDLE")]
    pub hehbbiiddle: String,

    #[serde(rename = "MANCMFGPCCJ")]
    pub mancmfgpccj: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MonsterRarity {
    #[serde(rename = "MONSTER_RARITY_BIG_BOSS_MONSTER")]
    MonsterRarityBigBossMonster,

    #[serde(rename = "MONSTER_RARITY_BOSS_MONSTER")]
    MonsterRarityBossMonster,

    #[serde(rename = "MONSTER_RARITY_ELITE_MONSTER")]
    MonsterRarityEliteMonster,

    #[serde(rename = "MONSTER_RARITY_SMALL_ENV_ANIMAL")]
    MonsterRaritySmallEnvAnimal,

    #[serde(rename = "MONSTER_RARITY_SMALL_MONSTER")]
    MonsterRaritySmallMonster,
}

pub fn load() -> Result<MonsterRelationshipExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MonsterRelationshipExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

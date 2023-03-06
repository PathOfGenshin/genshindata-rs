// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgGameRewardExcelConfigData = Vec<GcgGameRewardExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgGameRewardExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "NJKIONICHCH")]
    pub njkionichch: Vec<Njkionichch>,

    #[serde(rename = "levelNameTextMapHash")]
    pub level_name_text_map_hash: i64,

    #[serde(rename = "LCDFHLPBNFF")]
    pub lcdfhlpbnff: i64,

    #[serde(rename = "JFALOFICHDF")]
    pub jfalofichdf: Vec<i64>,

    #[serde(rename = "FNHKEJBPIBJ")]
    pub fnhkejbpibj: Vec<i64>,

    #[serde(rename = "CBAKGGNAMEL")]
    pub cbakggnamel: Vec<i64>,

    #[serde(rename = "failTips")]
    pub fail_tips: Vec<i64>,

    #[serde(rename = "LLJKOKNFJNL")]
    pub lljkoknfjnl: Option<Lljkoknfjnl>,

    #[serde(rename = "PLNANOLFFLN")]
    pub plnanolffln: i64,

    #[serde(rename = "COIPFLJOHBM")]
    pub coipfljohbm: Option<String>,

    #[serde(rename = "condList")]
    pub cond_list: Vec<CondList>,

    #[serde(rename = "PKDBOPGJOFB")]
    pub pkdbopgjofb: Pkdbopgjofb,

    #[serde(rename = "EHMGMFIDPPM")]
    pub ehmgmfidppm: Option<bool>,

    #[serde(rename = "groupId")]
    pub group_id: Option<i64>,

    #[serde(rename = "JDHEAGPAILJ")]
    pub jdheagpailj: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CondList {
    #[serde(rename = "paramList")]
    pub param_list: Vec<i64>,

    #[serde(rename = "type")]
    pub cond_list_type: Option<Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Njkionichch {
    #[serde(rename = "challengeId")]
    pub challenge_id: Option<i64>,

    #[serde(rename = "rewardId")]
    pub reward_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FINISH_LEVEL_CHALLENGE")]
    FinishLevelChallenge,

    #[serde(rename = "GCG_LEVEL")]
    GcgLevel,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Lljkoknfjnl {
    #[serde(rename = "PVE_MONSTER")]
    PveMonster,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Pkdbopgjofb {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "Gcg_Loading_Bg2")]
    GcgLoadingBg2,

    #[serde(rename = "Gcg_Loading_Bg3")]
    GcgLoadingBg3,

    #[serde(rename = "Gcg_Loading_Bg4")]
    GcgLoadingBg4,

    #[serde(rename = "Gcg_Loading_Bg5")]
    GcgLoadingBg5,
}

pub fn load() -> Result<GcgGameRewardExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGGameRewardExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

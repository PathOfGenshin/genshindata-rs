// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type FungusTrainingDungeonExcelConfigData = Vec<FungusTrainingDungeonExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FungusTrainingDungeonExcelConfigDatum {
    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "JCGOBEMLDFJ")]
    pub jcgobemldfj: i64,

    #[serde(rename = "BDHGILCKEAP")]
    pub bdhgilckeap: Bdhgilckeap,

    #[serde(rename = "EGKHABGENGP")]
    pub egkhabgengp: i64,

    #[serde(rename = "NBNJJCHBCOJ")]
    pub nbnjjchbcoj: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "NPPCJOALKIC")]
    pub nppcjoalkic: Vec<i64>,

    #[serde(rename = "DPDAKJEBBKD")]
    pub dpdakjebbkd: Vec<i64>,

    #[serde(rename = "JPODDNGDCIO")]
    pub jpoddngdcio: i64,

    #[serde(rename = "PMJOJJIAEDO")]
    pub pmjojjiaedo: i64,

    #[serde(rename = "AMINEIFEFDC")]
    pub amineifefdc: Vec<i64>,

    #[serde(rename = "baseScore")]
    pub base_score: f64,

    #[serde(rename = "HBPLDONPGOH")]
    pub hbpldonpgoh: Option<f64>,

    #[serde(rename = "IFOPNKJOCJF")]
    pub ifopnkjocjf: Option<f64>,

    #[serde(rename = "KMCHCKDJJHK")]
    pub kmchckdjjhk: f64,

    #[serde(rename = "LDFLPBNDCFH")]
    pub ldflpbndcfh: Vec<i64>,

    #[serde(rename = "FNMMDMMIBIN")]
    pub fnmmdmmibin: i64,

    #[serde(rename = "OCNGGMGIPAF")]
    pub ocnggmgipaf: i64,

    #[serde(rename = "OHBKNKMNHHL")]
    pub ohbknkmnhhl: i64,

    #[serde(rename = "LDKCOCKDLHD")]
    pub ldkcockdlhd: i64,

    #[serde(rename = "EMEMLGCENPB")]
    pub ememlgcenpb: Vec<i64>,

    #[serde(rename = "DCPBGLGKOJL")]
    pub dcpbglgkojl: Option<i64>,

    #[serde(rename = "BPKBLCMDJBG")]
    pub bpkblcmdjbg: Option<i64>,

    #[serde(rename = "JFPNLMPKFGG")]
    pub jfpnlmpkfgg: Option<i64>,

    #[serde(rename = "AMNNLOJGBFN")]
    pub amnnlojgbfn: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Bdhgilckeap {
    #[serde(rename = "FUNGUS_TRAINING_DUNGEON_ATTACK")]
    FungusTrainingDungeonAttack,

    #[serde(rename = "FUNGUS_TRAINING_DUNGEON_DEFEND")]
    FungusTrainingDungeonDefend,
}

pub fn load() -> Result<FungusTrainingDungeonExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "FungusTrainingDungeonExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

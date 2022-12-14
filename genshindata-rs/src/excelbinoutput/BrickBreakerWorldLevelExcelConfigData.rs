// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type BrickBreakerWorldLevelExcelConfigData = Vec<BrickBreakerWorldLevelExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BrickBreakerWorldLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "HJDKODMLFEM")]
    pub hjdkodmlfem: Option<i64>,

    #[serde(rename = "draftId")]
    pub draft_id: Option<i64>,

    #[serde(rename = "limitTime")]
    pub limit_time: i64,

    #[serde(rename = "NDMEONJHCIH")]
    pub ndmeonjhcih: i64,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "transportPointList")]
    pub transport_point_list: Vec<i64>,

    #[serde(rename = "durationList")]
    pub duration_list: Vec<i64>,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,

    #[serde(rename = "HKLBCGJAALH")]
    pub hklbcgjaalh: i64,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,

    #[serde(rename = "EGMOAJDFCDP")]
    pub egmoajdfcdp: i64,

    #[serde(rename = "FKIMIKJCEFI")]
    pub fkimikjcefi: Vec<i64>,

    #[serde(rename = "DMOOIIJHFBP")]
    pub dmooiijhfbp: Vec<Dmooiijhfbp>,

    #[serde(rename = "EPFCEJJIGMD")]
    pub epfcejjigmd: Vec<i64>,

    #[serde(rename = "GJOIKBNHEJM")]
    pub gjoikbnhejm: Option<bool>,

    #[serde(rename = "OOHDEGCFMFB")]
    pub oohdegcfmfb: Option<i64>,

    #[serde(rename = "NKAGCJEOBHE")]
    pub nkagcjeobhe: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Dmooiijhfbp {
    #[serde(rename = "Fire")]
    Fire,

    #[serde(rename = "None")]
    None,

    #[serde(rename = "Water")]
    Water,
}

pub fn load() -> Result<BrickBreakerWorldLevelExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BrickBreakerWorldLevelExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

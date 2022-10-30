// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type MusicRiddleConfigData = Vec<MusicRiddleConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicRiddleConfigDatum {
    #[serde(rename = "LNCMLPKLEDG")]
    pub lncmlpkledg: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "MCCIJIPGJLD")]
    pub mccijipgjld: i64,

    #[serde(rename = "NLJDBCFJOBJ")]
    pub nljdbcfjobj: Vec<i64>,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "JILJDJKFJFL")]
    pub jiljdjkfjfl: Option<i64>,

    #[serde(rename = "HALGLNBAGIJ")]
    pub halglnbagij: Option<i64>,

    #[serde(rename = "GADIOHMEDHC")]
    pub gadiohmedhc: Option<bool>,
}

pub fn load() -> Result<MusicRiddleConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "MusicRiddleConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

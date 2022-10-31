// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type MpPlayMatchExcelConfigData = Vec<MpPlayMatchExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MpPlayMatchExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "playType")]
    pub play_type: String,

    #[serde(rename = "playNameTextMapHash")]
    pub play_name_text_map_hash: i64,

    #[serde(rename = "playDescTextMapHash")]
    pub play_desc_text_map_hash: i64,

    #[serde(rename = "isAutoMatch")]
    pub is_auto_match: bool,

    #[serde(rename = "minPlayers")]
    pub min_players: i64,

    #[serde(rename = "maxPlayers")]
    pub max_players: i64,

    #[serde(rename = "isAllowInAnyTime")]
    pub is_allow_in_any_time: bool,

    #[serde(rename = "isMatchNecessary")]
    pub is_match_necessary: Option<bool>,

    #[serde(rename = "settleType")]
    pub settle_type: String,

    #[serde(rename = "buffList")]
    pub buff_list: Vec<i64>,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: Option<i64>,

    #[serde(rename = "bgImage")]
    pub bg_image: String,

    #[serde(rename = "CJOEIPPLEMM")]
    pub cjoeipplemm: Option<bool>,

    #[serde(rename = "MFIPOMCNJMN")]
    pub mfipomcnjmn: Option<i64>,
}

pub fn load() -> Result<MpPlayMatchExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "MpPlayMatchExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
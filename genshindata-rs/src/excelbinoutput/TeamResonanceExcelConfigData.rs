// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type TeamResonanceExcelConfigData = Vec<TeamResonanceExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamResonanceExcelConfigDatum {
    #[serde(rename = "teamResonanceId")]
    pub team_resonance_id: i64,

    #[serde(rename = "teamResonanceGroupId")]
    pub team_resonance_group_id: i64,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "fireAvatarCount")]
    pub fire_avatar_count: Option<i64>,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "openConfig")]
    pub open_config: String,

    #[serde(rename = "addProps")]
    pub add_props: Vec<Option<serde_json::Value>>,

    #[serde(rename = "paramList")]
    pub param_list: Vec<f64>,

    #[serde(rename = "waterAvatarCount")]
    pub water_avatar_count: Option<i64>,

    #[serde(rename = "windAvatarCount")]
    pub wind_avatar_count: Option<i64>,

    #[serde(rename = "electricAvatarCount")]
    pub electric_avatar_count: Option<i64>,

    #[serde(rename = "grassAvatarCount")]
    pub grass_avatar_count: Option<i64>,

    #[serde(rename = "iceAvatarCount")]
    pub ice_avatar_count: Option<i64>,

    #[serde(rename = "rockAvatarCount")]
    pub rock_avatar_count: Option<i64>,

    #[serde(rename = "cond")]
    pub cond: Option<String>,
}

pub fn load() -> Result<TeamResonanceExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "TeamResonanceExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

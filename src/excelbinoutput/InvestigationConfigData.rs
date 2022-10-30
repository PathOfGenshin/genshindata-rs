// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type InvestigationConfigData = Vec<InvestigationConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InvestigationConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "nextInvestigationIdList")]
    pub next_investigation_id_list: Vec<i64>,

    #[serde(rename = "unlockOpenStateType")]
    pub unlock_open_state_type: Option<String>,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "investigationType")]
    pub investigation_type: Option<String>,

    #[serde(rename = "unlockLevel")]
    pub unlock_level: Option<i64>,
}

pub fn load() -> Result<InvestigationConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "InvestigationConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

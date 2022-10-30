// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type OpenStateConfigData = Vec<OpenStateConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenStateConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "allowClientOpen")]
    pub allow_client_open: Option<bool>,

    #[serde(rename = "cond")]
    pub cond: Vec<Cond>,

    #[serde(rename = "defaultState")]
    pub default_state: Option<bool>,

    #[serde(rename = "systemOpenUiId")]
    pub system_open_ui_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "condType")]
    pub cond_type: Option<CondType>,

    #[serde(rename = "param")]
    pub param: Option<i64>,

    #[serde(rename = "param2")]
    pub param2: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "OPEN_STATE_CITY_REPUTATION_LEVEL")]
    OpenStateCityReputationLevel,

    #[serde(rename = "OPEN_STATE_COND_PARENT_QUEST")]
    OpenStateCondParentQuest,

    #[serde(rename = "OPEN_STATE_COND_PLAYER_LEVEL")]
    OpenStateCondPlayerLevel,

    #[serde(rename = "OPEN_STATE_COND_QUEST")]
    OpenStateCondQuest,

    #[serde(rename = "OPEN_STATE_OFFERING_LEVEL")]
    OpenStateOfferingLevel,
}

pub fn load() -> Result<OpenStateConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "OpenStateConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

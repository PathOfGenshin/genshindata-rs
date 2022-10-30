// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type SignInCondExcelConfigData = Vec<SignInCondExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInCondExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "condList")]
    pub cond_list: Vec<CondList>,

    #[serde(rename = "totalDayCount")]
    pub total_day_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CondList {
    #[serde(rename = "type")]
    pub cond_list_type: Option<Type>,

    #[serde(rename = "paramList")]
    pub param_list: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "SIGN_IN_COND_PLAYER_LEVEL")]
    SignInCondPlayerLevel,
}

pub fn load() -> Result<SignInCondExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "SignInCondExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type SignInPeriodExcelConfigData = Vec<SignInPeriodExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInPeriodExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "perfabPath")]
    pub perfab_path: String,
}

pub fn load() -> Result<SignInPeriodExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "SignInPeriodExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
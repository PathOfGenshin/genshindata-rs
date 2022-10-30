// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type IrodoriFlowerGroupExcelConfigData = Vec<IrodoriFlowerGroupExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct IrodoriFlowerGroupExcelConfigDatum {
    #[serde(rename = "groupId")]
    pub group_id: i64,
}

pub fn load() -> Result<IrodoriFlowerGroupExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "IrodoriFlowerGroupExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

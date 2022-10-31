// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type BlossomTalkExcelConfigData = Vec<BlossomTalkExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlossomTalkExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "refreshId")]
    pub refresh_id: i64,

    #[serde(rename = "talkId")]
    pub talk_id: Vec<i64>,

    #[serde(rename = "groupId")]
    pub group_id: Option<i64>,
}

pub fn load() -> Result<BlossomTalkExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "BlossomTalkExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type GivingGroupExcelConfigData = Vec<GivingGroupExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GivingGroupExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "ItemIds")]
    pub item_ids: Vec<i64>,

    #[serde(rename = "finishTalkId")]
    pub finish_talk_id: Option<i64>,

    #[serde(rename = "mistakeTalkId")]
    pub mistake_talk_id: Option<i64>,

    #[serde(rename = "HIDNEKHDODL")]
    pub hidnekhdodl: Option<i64>,
}

pub fn load() -> Result<GivingGroupExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "GivingGroupExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

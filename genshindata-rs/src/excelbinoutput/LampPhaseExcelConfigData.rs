// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type LampPhaseExcelConfigData = Vec<LampPhaseExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LampPhaseExcelConfigDatum {
    #[serde(rename = "phaseId")]
    pub phase_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "endProgress")]
    pub end_progress: Option<i64>,

    #[serde(rename = "materialVec")]
    pub material_vec: Vec<i64>,

    #[serde(rename = "GivingId")]
    pub giving_id: i64,

    #[serde(rename = "contribution")]
    pub contribution: i64,

    #[serde(rename = "addProgress")]
    pub add_progress: Option<i64>,

    #[serde(rename = "notifyGroupId")]
    pub notify_group_id: i64,

    #[serde(rename = "gadgetId")]
    pub gadget_id: Option<i64>,

    #[serde(rename = "isDisplay")]
    pub is_display: Option<bool>,
}

pub fn load() -> Result<LampPhaseExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "LampPhaseExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
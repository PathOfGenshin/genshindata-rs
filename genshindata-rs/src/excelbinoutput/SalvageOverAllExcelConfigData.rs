// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type SalvageOverAllExcelConfigData = Vec<SalvageOverAllExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SalvageOverAllExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "BDBJLBCLGPC")]
    pub bdbjlbclgpc: i64,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: i64,

    #[serde(rename = "guideQuestId")]
    pub guide_quest_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "regionCenter")]
    pub region_center: Vec<f64>,

    #[serde(rename = "regionRadius")]
    pub region_radius: i64,

    #[serde(rename = "JPMPCJDLMON")]
    pub jpmpcjdlmon: i64,

    #[serde(rename = "reminderId")]
    pub reminder_id: i64,

    #[serde(rename = "HLCAEHIHLFM")]
    pub hlcaehihlfm: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "JBPBFNCDBHK")]
    pub jbpbfncdbhk: i64,

    #[serde(rename = "FEJKLIANLPM")]
    pub fejklianlpm: i64,
}

pub fn load() -> Result<SalvageOverAllExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "SalvageOverAllExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
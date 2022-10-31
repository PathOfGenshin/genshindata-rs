// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type HuntingRegionExcelConfigData = Vec<HuntingRegionExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HuntingRegionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "centerPosList")]
    pub center_pos_list: Vec<f64>,

    #[serde(rename = "centerRadius")]
    pub center_radius: i64,

    #[serde(rename = "safeClueGroup")]
    pub safe_clue_group: Vec<i64>,

    #[serde(rename = "clueGroup")]
    pub clue_group: Vec<i64>,

    #[serde(rename = "safeDestinationGroup")]
    pub safe_destination_group: Vec<i64>,

    #[serde(rename = "destinationGroup")]
    pub destination_group: Vec<i64>,

    #[serde(rename = "regionInfoTextMapHash")]
    pub region_info_text_map_hash: i64,

    #[serde(rename = "AIHBPFCJKAG")]
    pub aihbpfcjkag: Vec<i64>,
}

pub fn load() -> Result<HuntingRegionExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "HuntingRegionExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
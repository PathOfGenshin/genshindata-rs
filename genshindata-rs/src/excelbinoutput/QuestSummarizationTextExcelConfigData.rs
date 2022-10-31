// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type QuestSummarizationTextExcelConfigData = Vec<QuestSummarizationTextExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestSummarizationTextExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "DescTextMapHash")]
    pub desc_text_map_hash: i64,
}

pub fn load() -> Result<QuestSummarizationTextExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "QuestSummarizationTextExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
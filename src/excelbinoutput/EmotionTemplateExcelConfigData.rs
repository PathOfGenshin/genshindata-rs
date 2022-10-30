// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type EmotionTemplateExcelConfigData = Vec<EmotionTemplateExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmotionTemplateExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "freeStyleId")]
    pub free_style_id: i64,

    #[serde(rename = "bubbleName")]
    pub bubble_name: String,

    #[serde(rename = "emotionPath")]
    pub emotion_path: String,

    #[serde(rename = "protectTime")]
    pub protect_time: f64,
}

pub fn load() -> Result<EmotionTemplateExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "EmotionTemplateExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

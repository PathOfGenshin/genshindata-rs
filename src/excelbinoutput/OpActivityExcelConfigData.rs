// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type OpActivityExcelConfigData = Vec<OpActivityExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpActivityExcelConfigDatum {
    #[serde(rename = "opActivityId")]
    pub op_activity_id: i64,

    #[serde(rename = "bonusType")]
    pub bonus_type: String,

    #[serde(rename = "bonusValue")]
    pub bonus_value: i64,

    #[serde(rename = "bonusList")]
    pub bonus_list: Vec<i64>,

    #[serde(rename = "icon")]
    pub icon: i64,

    #[serde(rename = "tabText")]
    pub tab_text: String,

    #[serde(rename = "textMapIdList")]
    pub text_map_id_list: Vec<String>,
}

pub fn load() -> Result<OpActivityExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "OpActivityExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

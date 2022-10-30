// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type InstableSprayBuffExcelConfigData = Vec<InstableSprayBuffExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstableSprayBuffExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "GPDAEIMDCBF")]
    pub gpdaeimdcbf: String,

    #[serde(rename = "AMMJJJIODCM")]
    pub ammjjjiodcm: String,

    #[serde(rename = "NBLOCEJHFCN")]
    pub nblocejhfcn: i64,

    #[serde(rename = "HPIKALMBHLL")]
    pub hpikalmbhll: i64,

    #[serde(rename = "elementType")]
    pub element_type: i64,

    #[serde(rename = "buffNameTextMapHash")]
    pub buff_name_text_map_hash: i64,

    #[serde(rename = "CPOAMLCBFAG")]
    pub cpoamlcbfag: i64,

    #[serde(rename = "JGOAAIOIKNG")]
    pub jgoaaioikng: Vec<String>,
}

pub fn load() -> Result<InstableSprayBuffExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "InstableSprayBuffExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

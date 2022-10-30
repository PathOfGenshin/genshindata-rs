// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type VintageMarketBargainExcelConfigData = Vec<VintageMarketBargainExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VintageMarketBargainExcelConfigDatum {
    #[serde(rename = "MCFGLCDCIBM")]
    pub mcfglcdcibm: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "EOLLEOHDNAH")]
    pub eolleohdnah: i64,

    #[serde(rename = "MFJDNLLMCKK")]
    pub mfjdnllmckk: i64,
}

pub fn load() -> Result<VintageMarketBargainExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "VintageMarketBargainExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type PriceTierConfigData = Vec<PriceTierConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceTierConfigDatum {
    #[serde(rename = "tierName")]
    pub tier_name: String,

    #[serde(rename = "vipPoint")]
    pub vip_point: Option<i64>,
}

pub fn load() -> Result<PriceTierConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "PriceTierConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
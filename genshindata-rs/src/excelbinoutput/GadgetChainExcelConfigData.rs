// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type GadgetChainExcelConfigData = Vec<GadgetChainExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GadgetChainExcelConfigDatum {
    #[serde(rename = "BHFAONDJLEA")]
    pub bhfaondjlea: i64,

    #[serde(rename = "MKKFGGCINAP")]
    pub mkkfggcinap: Option<i64>,

    #[serde(rename = "maxLevel")]
    pub max_level: i64,

    #[serde(rename = "buffList")]
    pub buff_list: Vec<i64>,

    #[serde(rename = "LMFPBFABOMM")]
    pub lmfpbfabomm: Option<bool>,
}

pub fn load() -> Result<GadgetChainExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "GadgetChainExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
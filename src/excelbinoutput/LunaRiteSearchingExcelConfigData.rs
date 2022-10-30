// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type LunaRiteSearchingExcelConfigData = Vec<LunaRiteSearchingExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LunaRiteSearchingExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "JDPGGCCINLP")]
    pub jdpggccinlp: String,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "IICLCFHFPPB")]
    pub iiclcfhfppb: i64,

    #[serde(rename = "LFHBNHPDLHD")]
    pub lfhbnhpdlhd: Vec<i64>,

    #[serde(rename = "regionCenter")]
    pub region_center: Vec<f64>,

    #[serde(rename = "HKEDHFCNEIH")]
    pub hkedhfcneih: i64,

    #[serde(rename = "NAHCOFHKGGL")]
    pub nahcofhkggl: i64,

    #[serde(rename = "regionRadius")]
    pub region_radius: f64,

    #[serde(rename = "HNJEDIOCHGB")]
    pub hnjediochgb: i64,

    #[serde(rename = "DGPLAEGCKPN")]
    pub dgplaegckpn: i64,

    #[serde(rename = "EFHHAPJKCPG")]
    pub efhhapjkcpg: i64,

    #[serde(rename = "AEGEKFJOKBD")]
    pub aegekfjokbd: i64,
}

pub fn load() -> Result<LunaRiteSearchingExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "LunaRiteSearchingExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

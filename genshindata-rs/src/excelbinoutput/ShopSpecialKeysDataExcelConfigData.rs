// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ShopSpecialKeysDataExcelConfigData = Vec<ShopSpecialKeysDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShopSpecialKeysDataExcelConfigDatum {
    #[serde(rename = "goodsId")]
    pub goods_id: i64,

    #[serde(rename = "POGMGKHHCBL")]
    pub pogmgkhhcbl: i64,
}

pub fn load() -> Result<ShopSpecialKeysDataExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ShopSpecialKeysDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
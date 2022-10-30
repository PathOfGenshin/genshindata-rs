// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type FurnitureMakeExcelConfigData = Vec<FurnitureMakeExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FurnitureMakeExcelConfigDatum {
    #[serde(rename = "configID")]
    pub config_id: i64,

    #[serde(rename = "furnitureItemID")]
    pub furniture_item_id: i64,

    #[serde(rename = "count")]
    pub count: i64,

    #[serde(rename = "exp")]
    pub exp: i64,

    #[serde(rename = "materialItems")]
    pub material_items: Vec<MaterialItem>,

    #[serde(rename = "maxAccelerateTime")]
    pub max_accelerate_time: i64,

    #[serde(rename = "makeTime")]
    pub make_time: i64,

    #[serde(rename = "quickFetchMaterialNum")]
    pub quick_fetch_material_num: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialItem {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

pub fn load() -> Result<FurnitureMakeExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "FurnitureMakeExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ActivityPlantFlowerDailyExcelConfigData = Vec<ActivityPlantFlowerDailyExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityPlantFlowerDailyExcelConfigDatum {
    #[serde(rename = "dailyConfigId")]
    pub daily_config_id: i64,

    #[serde(rename = "costItemList")]
    pub cost_item_list: Vec<CostItemList>,

    #[serde(rename = "rewardIdList")]
    pub reward_id_list: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostItemList {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

pub fn load() -> Result<ActivityPlantFlowerDailyExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivityPlantFlowerDailyExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
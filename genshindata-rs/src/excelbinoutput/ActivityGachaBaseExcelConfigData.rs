// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityGachaBaseExcelConfigData = Vec<ActivityGachaBaseExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityGachaBaseExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "materialId")]
    pub material_id: i64,

    #[serde(rename = "NGHCEMFDBJJ")]
    pub nghcemfdbjj: i64,

    #[serde(rename = "NNNGLCCBJOE")]
    pub nnnglccbjoe: i64,

    #[serde(rename = "GEJJHKEGHPI")]
    pub gejjhkeghpi: i64,

    #[serde(rename = "LJEHAEGHCIG")]
    pub ljehaeghcig: i64,

    #[serde(rename = "MBLDIBOPHAO")]
    pub mbldibophao: i64,

    #[serde(rename = "BFHIGDIHIFL")]
    pub bfhigdihifl: i64,

    #[serde(rename = "IJFOAFFBNAG")]
    pub ijfoaffbnag: i64,

    #[serde(rename = "FFFCCMJNIAH")]
    pub fffccmjniah: i64,

    #[serde(rename = "KIAMFEHECIF")]
    pub kiamfehecif: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "JNFJNEFPIBA")]
    pub jnfjnefpiba: Vec<i64>,

    #[serde(rename = "reminderId")]
    pub reminder_id: i64,

    #[serde(rename = "HALJCNFKMDD")]
    pub haljcnfkmdd: i64,

    #[serde(rename = "JLEFGJCGICI")]
    pub jlefgjcgici: i64,
}

pub fn load() -> Result<ActivityGachaBaseExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityGachaBaseExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

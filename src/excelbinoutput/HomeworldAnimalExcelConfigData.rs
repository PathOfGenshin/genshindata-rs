// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type HomeworldAnimalExcelConfigData = Vec<HomeworldAnimalExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeworldAnimalExcelConfigDatum {
    #[serde(rename = "furnitureID")]
    pub furniture_id: i64,

    #[serde(rename = "monsterID")]
    pub monster_id: i64,

    #[serde(rename = "isRebirth")]
    pub is_rebirth: i64,

    #[serde(rename = "rebirthCD")]
    pub rebirth_cd: i64,
}

pub fn load() -> Result<HomeworldAnimalExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "HomeworldAnimalExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

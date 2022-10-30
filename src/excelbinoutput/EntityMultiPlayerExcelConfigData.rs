// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type EntityMultiPlayerExcelConfigData = Vec<EntityMultiPlayerExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityMultiPlayerExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "propPerVec")]
    pub prop_per_vec: Vec<PropPerVec>,

    #[serde(rename = "endureNumVec")]
    pub endure_num_vec: Vec<i64>,

    #[serde(rename = "elementShieldPerVec")]
    pub element_shield_per_vec: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropPerVec {
    #[serde(rename = "propType")]
    pub prop_type: PropType,

    #[serde(rename = "propValueVec")]
    pub prop_value_vec: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PropType {
    #[serde(rename = "FIGHT_PROP_ATTACK_MP_PERCENT")]
    FightPropAttackMpPercent,

    #[serde(rename = "FIGHT_PROP_HP_MP_PERCENT")]
    FightPropHpMpPercent,
}

pub fn load() -> Result<EntityMultiPlayerExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "EntityMultiPlayerExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

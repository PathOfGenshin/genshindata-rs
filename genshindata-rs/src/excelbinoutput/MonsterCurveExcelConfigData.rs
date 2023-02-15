// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MonsterCurveExcelConfigData = Vec<MonsterCurveExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MonsterCurveExcelConfigDatum {
    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "curveInfos")]
    pub curve_infos: Vec<CurveInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurveInfo {
    #[serde(rename = "type")]
    pub curve_info_type: Type,

    #[serde(rename = "arith")]
    pub arith: Arith,

    #[serde(rename = "value")]
    pub value: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Arith {
    #[serde(rename = "ARITH_ADD")]
    ArithAdd,

    #[serde(rename = "ARITH_MULTI")]
    ArithMulti,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "GROW_CURVE_ATTACK")]
    GrowCurveAttack,

    #[serde(rename = "GROW_CURVE_ATTACK_2")]
    GrowCurveAttack2,

    #[serde(rename = "GROW_CURVE_DEFENSE")]
    GrowCurveDefense,

    #[serde(rename = "GROW_CURVE_ELEMENT")]
    GrowCurveElement,

    #[serde(rename = "GROW_CURVE_HP")]
    GrowCurveHp,

    #[serde(rename = "GROW_CURVE_HP_2")]
    GrowCurveHp2,

    #[serde(rename = "GROW_CURVE_HP_LITTLEMONSTER")]
    GrowCurveHpLittlemonster,

    #[serde(rename = "GROW_CURVE_KILL_EXP")]
    GrowCurveKillExp,

    #[serde(rename = "GROW_CURVE_MATK")]
    GrowCurveMatk,

    #[serde(rename = "GROW_CURVE_MHP")]
    GrowCurveMhp,

    #[serde(rename = "GROW_CURVE_STRIKE")]
    GrowCurveStrike,

    #[serde(rename = "GROW_CURVE_STRIKE_HURT")]
    GrowCurveStrikeHurt,
}

pub fn load() -> Result<MonsterCurveExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MonsterCurveExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

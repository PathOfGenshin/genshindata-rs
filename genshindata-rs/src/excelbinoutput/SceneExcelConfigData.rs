// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type SceneExcelConfigData = Vec<SceneExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "type")]
    pub scene_excel_config_datum_type: Type,

    #[serde(rename = "scriptData")]
    pub script_data: String,

    #[serde(rename = "overrideDefaultProfile")]
    pub override_default_profile: String,

    #[serde(rename = "levelEntityConfig")]
    pub level_entity_config: String,

    #[serde(rename = "specifiedAvatarList")]
    pub specified_avatar_list: Vec<i64>,

    #[serde(rename = "comment")]
    pub comment: Comment,

    #[serde(rename = "HLKDAIPGHGO")]
    pub hlkdaipghgo: Vec<i64>,

    #[serde(rename = "LJNMKKCMLOL")]
    pub ljnmkkcmlol: Option<bool>,

    #[serde(rename = "maxSpecifiedAvatarNum")]
    pub max_specified_avatar_num: Option<i64>,

    #[serde(rename = "FEICPPHODGE")]
    pub feicpphodge: Option<String>,

    #[serde(rename = "LIIICMJEHLG")]
    pub liiicmjehlg: Option<bool>,

    #[serde(rename = "LLBGGMJAGIE")]
    pub llbggmjagie: Option<i64>,

    #[serde(rename = "ILGKOJFHOKJ")]
    pub ilgkojfhokj: Option<bool>,

    #[serde(rename = "KKPHDMLHKOL")]
    pub kkphdmlhkol: Option<bool>,

    #[serde(rename = "BDDELKPADBA")]
    pub bddelkpadba: Option<f64>,

    #[serde(rename = "KHNIPCAPJFJ")]
    pub khnipcapjfj: Option<String>,

    #[serde(rename = "entityAppearSorted")]
    pub entity_appear_sorted: Option<i64>,

    #[serde(rename = "DCLGPLMLLIP")]
    pub dclgplmllip: Option<Dclgplmllip>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Comment {
    #[serde(rename = "正式")]
    Comment,

    #[serde(rename = "测试")]
    Empty,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Dclgplmllip {
    #[serde(rename = "SCENE_SUB_TYPE_PERSISTENT_DUNGEON")]
    SceneSubTypePersistentDungeon,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "SCENE_DUNGEON")]
    SceneDungeon,

    #[serde(rename = "SCENE_HOME_ROOM")]
    SceneHomeRoom,

    #[serde(rename = "SCENE_HOME_WORLD")]
    SceneHomeWorld,

    #[serde(rename = "SCENE_ROOM")]
    SceneRoom,

    #[serde(rename = "SCENE_WORLD")]
    SceneWorld,
}

pub fn load() -> Result<SceneExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "SceneExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
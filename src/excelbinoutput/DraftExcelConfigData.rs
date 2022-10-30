// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type DraftExcelConfigData = Vec<DraftExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DraftExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "transferConfig")]
    pub transfer_config: Vec<TransferConfig>,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "exec")]
    pub exec: Exec,

    #[serde(rename = "param")]
    pub param: Option<i64>,

    #[serde(rename = "enableMp")]
    pub enable_mp: bool,

    #[serde(rename = "isNeedAllAgree")]
    pub is_need_all_agree: bool,

    #[serde(rename = "confirmCountDown")]
    pub confirm_count_down: i64,

    #[serde(rename = "minPlayerCount")]
    pub min_player_count: i64,

    #[serde(rename = "isNeedTwiceConfirm")]
    pub is_need_twice_confirm: Option<bool>,

    #[serde(rename = "twiceConfirmCountDown")]
    pub twice_confirm_count_down: i64,

    #[serde(rename = "EDDMGJAGPMC")]
    pub eddmgjagpmc: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TransferConfig {
    #[serde(rename = "groupId")]
    pub group_id: Option<i64>,

    #[serde(rename = "configId")]
    pub config_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Exec {
    #[serde(rename = "DRAFT_EXEC_AUTOSTART_GALLERY")]
    DraftExecAutostartGallery,

    #[serde(rename = "DRAFT_EXEC_GALLERY")]
    DraftExecGallery,

    #[serde(rename = "DRAFT_EXEC_HIDE_AND_SEEK")]
    DraftExecHideAndSeek,
}

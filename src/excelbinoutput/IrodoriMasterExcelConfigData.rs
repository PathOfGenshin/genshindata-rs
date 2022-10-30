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

pub type IrodoriMasterExcelConfigData = Vec<IrodoriMasterExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct IrodoriMasterExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "levelID")]
    pub level_id: i64,

    #[serde(rename = "AHNJPLCJGMP")]
    pub ahnjplcjgmp: Ahnjplcjgmp,

    #[serde(rename = "HAFOEGJLOAK")]
    pub hafoegjloak: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "GGBMACHIKED")]
    pub ggbmachiked: i64,

    #[serde(rename = "MLFHNJDMDAD")]
    pub mlfhnjdmdad: i64,

    #[serde(rename = "condID")]
    pub cond_id: i64,

    #[serde(rename = "JGINECNPLJM")]
    pub jginecnpljm: i64,

    #[serde(rename = "HCNJELPODFD")]
    pub hcnjelpodfd: i64,

    #[serde(rename = "LPHCAOOMLAM")]
    pub lphcaoomlam: i64,

    #[serde(rename = "PDBHPEMOAAK")]
    pub pdbhpemoaak: i64,

    #[serde(rename = "FNHGKMOMHAB")]
    pub fnhgkmomhab: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Ahnjplcjgmp {
    #[serde(rename = "IRODORI_MASTER_LEVEL_HARD")]
    IrodoriMasterLevelHard,

    #[serde(rename = "IRODORI_MASTER_LEVEL_MASTER")]
    IrodoriMasterLevelMaster,

    #[serde(rename = "IRODORI_MASTER_LEVEL_NORAML")]
    IrodoriMasterLevelNoraml,
}

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

pub type ActivityGachaStageExcelConfigData = Vec<ActivityGachaStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityGachaStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "HLOAELINJKE")]
    pub hloaelinjke: i64,

    #[serde(rename = "CPDLLKMLCHN")]
    pub cpdllkmlchn: Option<bool>,

    #[serde(rename = "type")]
    pub activity_gacha_stage_excel_config_datum_type: String,

    #[serde(rename = "groupIdList")]
    pub group_id_list: Vec<i64>,

    #[serde(rename = "PFKKPFOLGJN")]
    pub pfkkpfolgjn: Vec<i64>,

    #[serde(rename = "JBEICIDFOEP")]
    pub jbeicidfoep: Vec<i64>,

    #[serde(rename = "LPDIHPENLFG")]
    pub lpdihpenlfg: Vec<i64>,

    #[serde(rename = "condID")]
    pub cond_id: Option<i64>,

    #[serde(rename = "watcherID")]
    pub watcher_id: Option<i64>,

    #[serde(rename = "DOPFPLFNJEG")]
    pub dopfplfnjeg: Option<i64>,

    #[serde(rename = "PCMJIABGLGO")]
    pub pcmjiabglgo: Option<bool>,
}

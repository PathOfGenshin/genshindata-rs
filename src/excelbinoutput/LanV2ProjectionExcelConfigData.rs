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

pub type LanV2ProjectionExcelConfigData = Vec<LanV2ProjectionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LanV2ProjectionExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "BJDNOKIJINC")]
    pub bjdnokijinc: f64,

    #[serde(rename = "KAPJBKEDJGG")]
    pub kapjbkedjgg: f64,

    #[serde(rename = "OCNAFIGKIIB")]
    pub ocnafigkiib: f64,

    #[serde(rename = "NMHGBHDDNLA")]
    pub nmhgbhddnla: f64,

    #[serde(rename = "GLPJPKKPOAN")]
    pub glpjpkkpoan: f64,

    #[serde(rename = "FDDHNPIBNMD")]
    pub fddhnpibnmd: f64,

    #[serde(rename = "BCAIINMJPKJ")]
    pub bcaiinmjpkj: i64,

    #[serde(rename = "MBDNFHIMCDI")]
    pub mbdnfhimcdi: i64,

    #[serde(rename = "FAPBEHGANGC")]
    pub fapbehgangc: i64,

    #[serde(rename = "LFMJDDHGNFD")]
    pub lfmjddhgnfd: i64,

    #[serde(rename = "BPAJIBLNIPL")]
    pub bpajiblnipl: i64,

    #[serde(rename = "ANNIBGDECOJ")]
    pub annibgdecoj: i64,

    #[serde(rename = "CNLHPGLBPKM")]
    pub cnlhpglbpkm: Vec<String>,

    #[serde(rename = "AFLCBAKOFEC")]
    pub aflcbakofec: Vec<String>,

    #[serde(rename = "guideQuestId")]
    pub guide_quest_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
}

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

pub type LoadingTipsExcelConfigData = Vec<LoadingTipsExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LoadingTipsExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "tipsTitleTextMapHash")]
    pub tips_title_text_map_hash: i64,

    #[serde(rename = "tipsDescTextMapHash")]
    pub tips_desc_text_map_hash: i64,

    #[serde(rename = "stageID")]
    pub stage_id: String,

    #[serde(rename = "startTime")]
    pub start_time: String,

    #[serde(rename = "endTime")]
    pub end_time: String,

    #[serde(rename = "limitOpenState")]
    pub limit_open_state: LimitOpenState,

    #[serde(rename = "preMainQuestIds")]
    pub pre_main_quest_ids: String,

    #[serde(rename = "OEGMFOHBAHC")]
    pub oegmfohbahc: Vec<i64>,

    #[serde(rename = "NFNEGJEAKBL")]
    pub nfnegjeakbl: Vec<i64>,

    #[serde(rename = "GFGPIGCAPFB")]
    pub gfgpigcapfb: Vec<Option<serde_json::Value>>,

    #[serde(rename = "weight")]
    pub weight: Option<i64>,

    #[serde(rename = "minLevel")]
    pub min_level: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum LimitOpenState {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "OPEN_STATE_COMBINE")]
    OpenStateCombine,

    #[serde(rename = "OPEN_STATE_GACHA")]
    OpenStateGacha,

    #[serde(rename = "OPEN_STATE_GUIDE_BLOSSOM")]
    OpenStateGuideBlossom,

    #[serde(rename = "OPEN_STATE_LOADINGTIPS_ENKANOMIYA")]
    OpenStateLoadingtipsEnkanomiya,
}

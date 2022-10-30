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

pub type ActivitySalesmanDailyExcelConfigData = Vec<ActivitySalesmanDailyExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivitySalesmanDailyExcelConfigDatum {
    #[serde(rename = "dailyConfigId")]
    pub daily_config_id: i64,

    #[serde(rename = "costItemList")]
    pub cost_item_list: Vec<CostItemList>,

    #[serde(rename = "clusPosTextMapHash")]
    pub clus_pos_text_map_hash: i64,

    #[serde(rename = "npcTalkTextMapHash")]
    pub npc_talk_text_map_hash: i64,

    #[serde(rename = "IntroTextMapHash")]
    pub intro_text_map_hash: i64,

    #[serde(rename = "tracePosition")]
    pub trace_position: String,
}

#[derive(Serialize, Deserialize)]
pub struct CostItemList {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

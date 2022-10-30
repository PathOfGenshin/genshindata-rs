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

pub type VintageMarketNpcEventExcelConfigData = Vec<VintageMarketNpcEventExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct VintageMarketNpcEventExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "talkId")]
    pub talk_id: i64,

    #[serde(rename = "npcId")]
    pub npc_id: i64,

    #[serde(rename = "duration")]
    pub duration: i64,

    #[serde(rename = "OOHLELDFICF")]
    pub oohleldficf: Vec<i64>,

    #[serde(rename = "KJOHBJDNJAI")]
    pub kjohbjdnjai: Vec<Kjohbjdnjai>,

    #[serde(rename = "FDAEPIFDIDO")]
    pub fdaepifdido: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Kjohbjdnjai {
    #[serde(rename = "talkId")]
    pub talk_id: Option<i64>,

    #[serde(rename = "rewardId")]
    pub reward_id: Option<i64>,
}

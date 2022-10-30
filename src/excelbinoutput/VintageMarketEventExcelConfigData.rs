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

pub type VintageMarketEventExcelConfigData = Vec<VintageMarketEventExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct VintageMarketEventExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "triggerType")]
    pub trigger_type: TriggerType,

    #[serde(rename = "KONAGJHFPNH")]
    pub konagjhfpnh: String,

    #[serde(rename = "KJJLOFLLCAM")]
    pub kjjlofllcam: Kjjlofllcam,

    #[serde(rename = "KFHMLBNPIGF")]
    pub kfhmlbnpigf: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Kjjlofllcam {
    #[serde(rename = "VINTAGE_MARKET_EVENT_ENV")]
    VintageMarketEventEnv,

    #[serde(rename = "VINTAGE_MARKET_EVENT_NPC")]
    VintageMarketEventNpc,
}

#[derive(Serialize, Deserialize)]
pub enum TriggerType {
    #[serde(rename = "VINTAGE_MARKET_EVENT_TRIGGER_ROUND")]
    VintageMarketEventTriggerRound,
}

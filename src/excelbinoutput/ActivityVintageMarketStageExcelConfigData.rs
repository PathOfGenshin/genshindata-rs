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

pub type ActivityVintageMarketStageExcelConfigData =
    Vec<ActivityVintageMarketStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityVintageMarketStageExcelConfigDatum {
    #[serde(rename = "stageID")]
    pub stage_id: i64,

    #[serde(rename = "LCPLPHEEAAL")]
    pub lcplpheeaal: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "DOPFPLFNJEG")]
    pub dopfplfnjeg: i64,

    #[serde(rename = "MGPELOMDGHE")]
    pub mgpelomdghe: i64,

    #[serde(rename = "NDMBFNHCNGG")]
    pub ndmbfnhcngg: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "DJKCKMEMANA")]
    pub djkckmemana: i64,

    #[serde(rename = "BAPBEPLJNLO")]
    pub bapbepljnlo: i64,

    #[serde(rename = "GMDIFFDDHDC")]
    pub gmdiffddhdc: Option<i64>,
}

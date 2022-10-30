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

pub type IrodoriPoetryExcelConfigData = Vec<IrodoriPoetryExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct IrodoriPoetryExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "condID")]
    pub cond_id: i64,

    #[serde(rename = "LHICMKKPEOO")]
    pub lhicmkkpeoo: i64,

    #[serde(rename = "HAFOEGJLOAK")]
    pub hafoegjloak: i64,

    #[serde(rename = "entityType")]
    pub entity_type: String,

    #[serde(rename = "CMECOBJCEEG")]
    pub cmecobjceeg: Vec<Cmecobjceeg>,

    #[serde(rename = "MPJOBAMPMPP")]
    pub mpjobampmpp: i64,

    #[serde(rename = "BAPMODDGGIP")]
    pub bapmoddggip: i64,

    #[serde(rename = "CCDMHAGGCCK")]
    pub ccdmhaggcck: i64,

    #[serde(rename = "EGBHFDJBCAB")]
    pub egbhfdjbcab: Vec<i64>,

    #[serde(rename = "watcherID")]
    pub watcher_id: i64,

    #[serde(rename = "PLMJBCDJHHA")]
    pub plmjbcdjhha: i64,

    #[serde(rename = "JILNEOHCOHP")]
    pub jilneohcohp: i64,

    #[serde(rename = "JMOEDAINFKE")]
    pub jmoedainfke: i64,

    #[serde(rename = "LGDJFFDJGCL")]
    pub lgdjffdjgcl: i64,

    #[serde(rename = "KHEDKNOOMDC")]
    pub khedknoomdc: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Cmecobjceeg {
    #[serde(rename = "AEKMNCGECNO")]
    pub aekmncgecno: Vec<i64>,

    #[serde(rename = "EPEDECCNOLI")]
    pub epedeccnoli: Option<i64>,
}

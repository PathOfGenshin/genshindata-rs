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

pub type ActivityHachiStageExcelConfigData = Vec<ActivityHachiStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityHachiStageExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "questId")]
    pub quest_id: Vec<i64>,

    #[serde(rename = "DGGMIFJMJBP")]
    pub dggmifjmjbp: i64,

    #[serde(rename = "HMMIBLBGDAN")]
    pub hmmiblbgdan: i64,

    #[serde(rename = "LDGIHBMIBLA")]
    pub ldgihbmibla: i64,

    #[serde(rename = "LPHCAOOMLAM")]
    pub lphcaoomlam: i64,

    #[serde(rename = "EBPOEAEADCF")]
    pub ebpoeaeadcf: i64,

    #[serde(rename = "NBMFBEIAHLI")]
    pub nbmfbeiahli: i64,

    #[serde(rename = "OOPMIEEFOEJ")]
    pub oopmieefoej: String,

    #[serde(rename = "PAEJCMHIEFO")]
    pub paejcmhiefo: String,

    #[serde(rename = "DDLAEIMBNCK")]
    pub ddlaeimbnck: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "BIPPDPDDPNA")]
    pub bippdpddpna: Vec<f64>,

    #[serde(rename = "HOMLCKOHNDN")]
    pub homlckohndn: Vec<f64>,

    #[serde(rename = "ABOEJKIDFLG")]
    pub aboejkidflg: Vec<f64>,

    #[serde(rename = "KFDAIBEJHNN")]
    pub kfdaibejhnn: Vec<i64>,

    #[serde(rename = "BDDKGECHJHO")]
    pub bddkgechjho: Vec<i64>,

    #[serde(rename = "ADIOLDLMEIF")]
    pub adioldlmeif: i64,

    #[serde(rename = "JGPNJLLGMMO")]
    pub jgpnjllgmmo: i64,

    #[serde(rename = "KGMMBFKGLAL")]
    pub kgmmbfkglal: i64,

    #[serde(rename = "IHLMALENLHO")]
    pub ihlmalenlho: i64,
}

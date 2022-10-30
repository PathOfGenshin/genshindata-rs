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

pub type ActivityGachaBaseExcelConfigData = Vec<ActivityGachaBaseExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityGachaBaseExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "materialId")]
    pub material_id: i64,

    #[serde(rename = "KBFOBKOPEAC")]
    pub kbfobkopeac: i64,

    #[serde(rename = "DIAIKJPHHAM")]
    pub diaikjphham: i64,

    #[serde(rename = "POMENCNEJPK")]
    pub pomencnejpk: i64,

    #[serde(rename = "GLDMPIOLLNG")]
    pub gldmpiollng: i64,

    #[serde(rename = "OCIGCGIJBEN")]
    pub ocigcgijben: i64,

    #[serde(rename = "HLKCHPOCPGI")]
    pub hlkchpocpgi: i64,

    #[serde(rename = "OGBAHDLFCOD")]
    pub ogbahdlfcod: i64,

    #[serde(rename = "ABEMAKPKNHG")]
    pub abemakpknhg: i64,

    #[serde(rename = "LAIDJIDPIHE")]
    pub laidjidpihe: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "LFLIICPCIKO")]
    pub lfliicpciko: Vec<i64>,

    #[serde(rename = "reminderId")]
    pub reminder_id: i64,

    #[serde(rename = "FJLAEPOKOJA")]
    pub fjlaepokoja: i64,

    #[serde(rename = "DLCJPJLDNLK")]
    pub dlcjpjldnlk: i64,
}

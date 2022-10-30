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

pub type ReliquarySetExcelConfigData = Vec<ReliquarySetExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReliquarySetExcelConfigDatum {
    #[serde(rename = "setId")]
    pub set_id: i64,

    #[serde(rename = "setIcon")]
    pub set_icon: String,

    #[serde(rename = "setNeedNum")]
    pub set_need_num: Vec<i64>,

    #[serde(rename = "EquipAffixId")]
    pub equip_affix_id: Option<i64>,

    #[serde(rename = "containsList")]
    pub contains_list: Vec<i64>,

    #[serde(rename = "FDKHPHNGCBL")]
    pub fdkhphngcbl: Option<i64>,

    #[serde(rename = "JCLGCACJMMK")]
    pub jclgcacjmmk: Vec<i64>,

    #[serde(rename = "textList")]
    pub text_list: Vec<i64>,

    #[serde(rename = "DisableFilter")]
    pub disable_filter: Option<i64>,
}

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

pub type EquipAffixExcelConfigData = Vec<EquipAffixExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct EquipAffixExcelConfigDatum {
    #[serde(rename = "affixId")]
    pub affix_id: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "openConfig")]
    pub open_config: String,

    #[serde(rename = "addProps")]
    pub add_props: Vec<AddProp>,

    #[serde(rename = "paramList")]
    pub param_list: Vec<f64>,

    #[serde(rename = "level")]
    pub level: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct AddProp {
    #[serde(rename = "propType")]
    pub prop_type: Option<String>,

    #[serde(rename = "value")]
    pub value: Option<f64>,
}

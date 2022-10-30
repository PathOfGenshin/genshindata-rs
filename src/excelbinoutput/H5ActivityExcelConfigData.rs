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

pub type H5ActivityExcelConfigData = Vec<H5ActivityExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct H5ActivityExcelConfigDatum {
    #[serde(rename = "h5ActivityId")]
    pub h5_activity_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "detailTextMapHash")]
    pub detail_text_map_hash: i64,

    #[serde(rename = "condList")]
    pub cond_list: Vec<CondList>,

    #[serde(rename = "condComb")]
    pub cond_comb: Option<CondComb>,
}

#[derive(Serialize, Deserialize)]
pub struct CondList {
    #[serde(rename = "paramStr")]
    pub param_str: String,

    #[serde(rename = "type")]
    pub cond_list_type: Option<Type>,
}

#[derive(Serialize, Deserialize)]
pub enum CondComb {
    #[serde(rename = "LOGIC_AND")]
    LogicAnd,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "H5_ACTIVITY_COND_PLAYER_LEVEL")]
    H5ActivityCondPlayerLevel,
}

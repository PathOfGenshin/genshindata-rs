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

pub type OpActivityExcelConfigData = Vec<OpActivityExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct OpActivityExcelConfigDatum {
    #[serde(rename = "opActivityId")]
    pub op_activity_id: i64,

    #[serde(rename = "bonusType")]
    pub bonus_type: String,

    #[serde(rename = "bonusValue")]
    pub bonus_value: i64,

    #[serde(rename = "bonusList")]
    pub bonus_list: Vec<i64>,

    #[serde(rename = "icon")]
    pub icon: i64,

    #[serde(rename = "tabText")]
    pub tab_text: String,

    #[serde(rename = "textMapIdList")]
    pub text_map_id_list: Vec<String>,
}

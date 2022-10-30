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

pub type WidgetExcelConfigData = Vec<WidgetExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WidgetExcelConfigDatum {
    #[serde(rename = "materialID")]
    pub material_id: i64,

    #[serde(rename = "customizeDesc")]
    pub customize_desc: Vec<i64>,

    #[serde(rename = "jsonAddr")]
    pub json_addr: Option<bool>,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: Option<i64>,
}

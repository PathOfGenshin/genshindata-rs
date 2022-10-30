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

pub type WidgetCameraExcelConfigData = Vec<WidgetCameraExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WidgetCameraExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "GHJHBKJGOJN")]
    pub ghjhbkjgojn: f64,

    #[serde(rename = "KAKDIKKLMIG")]
    pub kakdikklmig: String,

    #[serde(rename = "GKLDBFFOICP")]
    pub gkldbffoicp: i64,

    #[serde(rename = "BPDNBBNIGCC")]
    pub bpdnbbnigcc: i64,

    #[serde(rename = "OIJNJOFEPKL")]
    pub oijnjofepkl: i64,
}

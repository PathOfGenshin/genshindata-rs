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

pub type ActivityChessScheduleExcelConfigData = Vec<ActivityChessScheduleExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityChessScheduleExcelConfigDatum {
    #[serde(rename = "LCPIMDOENCB")]
    pub lcpimdoencb: i64,

    #[serde(rename = "GLEJLKJJGHJ")]
    pub glejlkjjghj: Option<i64>,

    #[serde(rename = "openMapList")]
    pub open_map_list: Vec<i64>,
}

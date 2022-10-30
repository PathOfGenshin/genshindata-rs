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

pub type HomeworldNpcExcelDataData = Vec<HomeworldNpcExcelDataDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeworldNpcExcelDataDatum {
    #[serde(rename = "AvatarID")]
    pub avatar_id: i64,

    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "FurnitureID")]
    pub furniture_id: i64,

    #[serde(rename = "PJAPBPDMLAE")]
    pub pjapbpdmlae: i64,

    #[serde(rename = "MJJBGINPDEE")]
    pub mjjbginpdee: Vec<i64>,
}

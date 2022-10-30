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

pub type CustomLevelDungeonConfigData = Vec<CustomLevelDungeonConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CustomLevelDungeonConfigDatum {
    #[serde(rename = "dungeonID")]
    pub dungeon_id: i64,

    #[serde(rename = "order")]
    pub order: i64,

    #[serde(rename = "jsonPath")]
    pub json_path: String,

    #[serde(rename = "BIDIIACHMFL")]
    pub bidiiachmfl: String,

    #[serde(rename = "CMFDFKNKHLL")]
    pub cmfdfknkhll: i64,

    #[serde(rename = "GEMJMGIENIE")]
    pub gemjmgienie: i64,

    #[serde(rename = "OBHIEGIMPHP")]
    pub obhiegimphp: i64,

    #[serde(rename = "FAHHBFIIIPM")]
    pub fahhbfiiipm: i64,

    #[serde(rename = "JNHDFJPBDKB")]
    pub jnhdfjpbdkb: i64,

    #[serde(rename = "HFPIHGPHHKM")]
    pub hfpihgphhkm: i64,

    #[serde(rename = "MAHNJHBNHOD")]
    pub mahnjhbnhod: String,

    #[serde(rename = "KLIINBHPGAK")]
    pub kliinbhpgak: Vec<i64>,
}

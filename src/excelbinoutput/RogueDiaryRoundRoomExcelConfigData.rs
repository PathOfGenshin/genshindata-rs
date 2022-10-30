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

pub type RogueDiaryRoundRoomExcelConfigData = Vec<RogueDiaryRoundRoomExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueDiaryRoundRoomExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "GJANGOGMLPD")]
    pub gjangogmlpd: Option<i64>,

    #[serde(rename = "PBEOIKIDLFP")]
    pub pbeoikidlfp: Vec<Pbeoikidlfp>,
}

#[derive(Serialize, Deserialize)]
pub struct Pbeoikidlfp {
    #[serde(rename = "FLPPMMOHLBP")]
    pub flppmmohlbp: Vec<i64>,

    #[serde(rename = "CICECADHLDG")]
    pub cicecadhldg: Vec<i64>,
}

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

pub type RoomExcelConfigData = Vec<RoomExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RoomExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "prefabPath")]
    pub prefab_path: String,

    #[serde(rename = "navMeshPath")]
    pub nav_mesh_path: String,
}

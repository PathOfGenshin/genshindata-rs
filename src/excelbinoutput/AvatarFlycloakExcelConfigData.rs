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

pub type AvatarFlycloakExcelConfigData = Vec<AvatarFlycloakExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AvatarFlycloakExcelConfigDatum {
    #[serde(rename = "flycloakId")]
    pub flycloak_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "prefabPath")]
    pub prefab_path: String,

    #[serde(rename = "jsonName")]
    pub json_name: JsonName,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "materialId")]
    pub material_id: i64,

    #[serde(rename = "hide")]
    pub hide: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum JsonName {
    #[serde(rename = "Flycloak_Default_01")]
    FlycloakDefault01,
}

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

pub type MaterialCodexExcelConfigData = Vec<MaterialCodexExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MaterialCodexExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "materialId")]
    pub material_id: i64,

    #[serde(rename = "SortOrder")]
    pub sort_order: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "isDisuse")]
    pub is_disuse: Option<bool>,

    #[serde(rename = "showOnlyUnlocked")]
    pub show_only_unlocked: Option<bool>,

    #[serde(rename = "type")]
    pub material_codex_excel_config_datum_type: Option<Type>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "CODEX_COOKING_FOOD")]
    CodexCookingFood,

    #[serde(rename = "CODEX_WAR_TROPHIES")]
    CodexWarTrophies,

    #[serde(rename = "CODEX_WIDGET")]
    CodexWidget,
}

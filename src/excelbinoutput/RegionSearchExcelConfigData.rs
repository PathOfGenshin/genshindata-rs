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

pub type RegionSearchExcelConfigData = Vec<RegionSearchExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RegionSearchExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "searchNameTextMapHash")]
    pub search_name_text_map_hash: i64,

    #[serde(rename = "searchDescTextMapHash")]
    pub search_desc_text_map_hash: i64,

    #[serde(rename = "searchCompleteTextMapHash")]
    pub search_complete_text_map_hash: i64,

    #[serde(rename = "tutorialId")]
    pub tutorial_id: i64,

    #[serde(rename = "markIconTypeName")]
    pub mark_icon_type_name: String,

    #[serde(rename = "searchType")]
    pub search_type: String,

    #[serde(rename = "searchGroupId")]
    pub search_group_id: i64,

    #[serde(rename = "reviseLevel")]
    pub revise_level: i64,

    #[serde(rename = "abilityGroup")]
    pub ability_group: String,
}

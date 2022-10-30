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

pub type BlossomGroupsExcelConfigData = Vec<BlossomGroupsExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BlossomGroupsExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "sectionId")]
    pub section_id: i64,

    #[serde(rename = "refreshTypeVec")]
    pub refresh_type_vec: Vec<i64>,

    #[serde(rename = "newGroupVec")]
    pub new_group_vec: Vec<i64>,

    #[serde(rename = "decorateGroupVec")]
    pub decorate_group_vec: Vec<i64>,

    #[serde(rename = "nextCampIdVec")]
    pub next_camp_id_vec: Vec<i64>,

    #[serde(rename = "isSafe")]
    pub is_safe: Option<bool>,

    #[serde(rename = "isInitialRefresh")]
    pub is_initial_refresh: Option<bool>,

    #[serde(rename = "finishProgress")]
    pub finish_progress: i64,

    #[serde(rename = "limitLevel")]
    pub limit_level: i64,

    #[serde(rename = "fightRadius")]
    pub fight_radius: i64,

    #[serde(rename = "remindRadius")]
    pub remind_radius: i64,

    #[serde(rename = "blossomTipsTextMapHash")]
    pub blossom_tips_text_map_hash: i64,

    #[serde(rename = "delayUnloadSec")]
    pub delay_unload_sec: Option<i64>,
}

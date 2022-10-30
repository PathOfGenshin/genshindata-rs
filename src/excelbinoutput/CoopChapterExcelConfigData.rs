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

pub type CoopChapterExcelConfigData = Vec<CoopChapterExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CoopChapterExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "chapterNameTextMapHash")]
    pub chapter_name_text_map_hash: i64,

    #[serde(rename = "coopPageTitleTextMapHash")]
    pub coop_page_title_text_map_hash: i64,

    #[serde(rename = "chapterSortId")]
    pub chapter_sort_id: i64,

    #[serde(rename = "avatarSortId")]
    pub avatar_sort_id: i64,

    #[serde(rename = "chapterIcon")]
    pub chapter_icon: String,

    #[serde(rename = "unlockCond")]
    pub unlock_cond: Vec<UnlockCond>,

    #[serde(rename = "unlockCondTips")]
    pub unlock_cond_tips: Vec<i64>,

    #[serde(rename = "openMaterialId")]
    pub open_material_id: i64,

    #[serde(rename = "openMaterialNum")]
    pub open_material_num: i64,

    #[serde(rename = "beginTimeStr")]
    pub begin_time_str: String,

    #[serde(rename = "confidenceValue")]
    pub confidence_value: i64,

    #[serde(rename = "pointGraphPath")]
    pub point_graph_path: String,

    #[serde(rename = "graphXRatio")]
    pub graph_x_ratio: f64,

    #[serde(rename = "graphYRatio")]
    pub graph_y_ratio: f64,
}

#[derive(Serialize, Deserialize)]
pub struct UnlockCond {
    #[serde(rename = "condType")]
    pub cond_type: CondType,

    #[serde(rename = "args")]
    pub args: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "COOP_COND_CHAPTER_END_ALL_FINISH")]
    CoopCondChapterEndAllFinish,

    #[serde(rename = "COOP_COND_FINISH_QUEST")]
    CoopCondFinishQuest,

    #[serde(rename = "COOP_COND_PLAYER_LEVEL")]
    CoopCondPlayerLevel,
}

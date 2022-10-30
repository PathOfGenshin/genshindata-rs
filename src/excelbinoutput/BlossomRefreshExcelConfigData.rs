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

pub type BlossomRefreshExcelConfigData = Vec<BlossomRefreshExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BlossomRefreshExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "refreshType")]
    pub refresh_type: String,

    #[serde(rename = "refreshCount")]
    pub refresh_count: i64,

    #[serde(rename = "refreshTime")]
    pub refresh_time: String,

    #[serde(rename = "refreshCondVec")]
    pub refresh_cond_vec: Vec<RefreshCondVec>,

    #[serde(rename = "reviseLevel")]
    pub revise_level: i64,

    #[serde(rename = "blossomChestId")]
    pub blossom_chest_id: Option<i64>,

    #[serde(rename = "campUpdateNeedCount")]
    pub camp_update_need_count: Option<i64>,

    #[serde(rename = "dropVec")]
    pub drop_vec: Vec<DropVec>,

    #[serde(rename = "clientShowType")]
    pub client_show_type: ClientShowType,

    #[serde(rename = "itemLimitType")]
    pub item_limit_type: Option<String>,

    #[serde(rename = "rewardType")]
    pub reward_type: Option<String>,

    #[serde(rename = "MADIKBJCJMJ")]
    pub madikbjcjmj: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct DropVec {
    #[serde(rename = "dropId")]
    pub drop_id: i64,

    #[serde(rename = "previewReward")]
    pub preview_reward: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshCondVec {
    #[serde(rename = "type")]
    pub refresh_cond_vec_type: Option<Type>,

    #[serde(rename = "param")]
    pub param: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum ClientShowType {
    #[serde(rename = "BLOSSOM_SHOWTYPE_CHALLENGE")]
    BlossomShowtypeChallenge,

    #[serde(rename = "BLOSSOM_SHOWTYPE_GROUPCHALLENGE")]
    BlossomShowtypeGroupchallenge,

    #[serde(rename = "BLOSSOM_SHOWTYPE_NPCTALK")]
    BlossomShowtypeNpctalk,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "BLOSSOM_REFRESH_COND_ACTIVITY_COND")]
    BlossomRefreshCondActivityCond,

    #[serde(rename = "BLOSSOM_REFRESH_COND_OPEN_STATE")]
    BlossomRefreshCondOpenState,

    #[serde(rename = "BLOSSOM_REFRESH_COND_PLAYER_LEVEL_EQUAL_GREATER")]
    BlossomRefreshCondPlayerLevelEqualGreater,

    #[serde(rename = "BLOSSOM_REFRESH_COND_PLAYER_LEVEL_LESS_THAN")]
    BlossomRefreshCondPlayerLevelLessThan,

    #[serde(rename = "BLOSSOM_REFRESH_COND_SCENE_TAG_ADDED")]
    BlossomRefreshCondSceneTagAdded,
}

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

pub type ForgeExcelConfigData = Vec<ForgeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ForgeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "playerLevel")]
    pub player_level: i64,

    #[serde(rename = "isDefaultShow")]
    pub is_default_show: Option<bool>,

    #[serde(rename = "effectiveWorldLevels")]
    pub effective_world_levels: Vec<i64>,

    #[serde(rename = "forgeType")]
    pub forge_type: i64,

    #[serde(rename = "showItemId")]
    pub show_item_id: i64,

    #[serde(rename = "HAHBBIAFIBE")]
    pub hahbbiafibe: Option<i64>,

    #[serde(rename = "resultItemId")]
    pub result_item_id: Option<i64>,

    #[serde(rename = "resultItemCount")]
    pub result_item_count: i64,

    #[serde(rename = "forgeTime")]
    pub forge_time: i64,

    #[serde(rename = "queueNum")]
    pub queue_num: i64,

    #[serde(rename = "scoinCost")]
    pub scoin_cost: i64,

    #[serde(rename = "randomItems")]
    pub random_items: Vec<RandomItem>,

    #[serde(rename = "materialItems")]
    pub material_items: Vec<MaterialItem>,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "forgePoint")]
    pub forge_point: Option<i64>,

    #[serde(rename = "forgePointNoticeTextMapHash")]
    pub forge_point_notice_text_map_hash: i64,

    #[serde(rename = "mainRandomDropId")]
    pub main_random_drop_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialItem {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct RandomItem {}

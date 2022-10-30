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

pub type AnimalCodexExcelConfigData = Vec<AnimalCodexExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AnimalCodexExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "type")]
    pub animal_codex_excel_config_datum_type: Option<Type>,

    #[serde(rename = "describeId")]
    pub describe_id: i64,

    #[serde(rename = "SortOrder")]
    pub sort_order: i64,

    #[serde(rename = "EDJJMOJHAFE")]
    pub edjjmojhafe: Option<Edjjmojhafe>,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "modelPath")]
    pub model_path: String,

    #[serde(rename = "pushTipsCodexId")]
    pub push_tips_codex_id: Option<i64>,

    #[serde(rename = "subType")]
    pub sub_type: Option<SubType>,

    #[serde(rename = "isDisuse")]
    pub is_disuse: Option<bool>,

    #[serde(rename = "isSeenActive")]
    pub is_seen_active: Option<bool>,

    #[serde(rename = "showOnlyUnlocked")]
    pub show_only_unlocked: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "CODEX_MONSTER")]
    CodexMonster,
}

#[derive(Serialize, Deserialize)]
pub enum Edjjmojhafe {
    #[serde(rename = "CODEX_COUNT_TYPE_CAPTURE")]
    CodexCountTypeCapture,

    #[serde(rename = "CODEX_COUNT_TYPE_FISH")]
    CodexCountTypeFish,

    #[serde(rename = "CODEX_COUNT_TYPE_KILL")]
    CodexCountTypeKill,
}

#[derive(Serialize, Deserialize)]
pub enum SubType {
    #[serde(rename = "CODEX_SUBTYPE_ABYSS")]
    CodexSubtypeAbyss,

    #[serde(rename = "CODEX_SUBTYPE_ANIMAL")]
    CodexSubtypeAnimal,

    #[serde(rename = "CODEX_SUBTYPE_AUTOMATRON")]
    CodexSubtypeAutomatron,

    #[serde(rename = "CODEX_SUBTYPE_AVIARY")]
    CodexSubtypeAviary,

    #[serde(rename = "CODEX_SUBTYPE_BEAST")]
    CodexSubtypeBeast,

    #[serde(rename = "CODEX_SUBTYPE_BOSS")]
    CodexSubtypeBoss,

    #[serde(rename = "CODEX_SUBTYPE_CRITTER")]
    CodexSubtypeCritter,

    #[serde(rename = "CODEX_SUBTYPE_FATUI")]
    CodexSubtypeFatui,

    #[serde(rename = "CODEX_SUBTYPE_FISH")]
    CodexSubtypeFish,

    #[serde(rename = "CODEX_SUBTYPE_HILICHURL")]
    CodexSubtypeHilichurl,

    #[serde(rename = "CODEX_SUBTYPE_HUMAN")]
    CodexSubtypeHuman,
}

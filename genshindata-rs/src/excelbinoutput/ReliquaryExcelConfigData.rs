// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ReliquaryExcelConfigData = Vec<ReliquaryExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReliquaryExcelConfigDatum {
    #[serde(rename = "equipType")]
    pub equip_type: EquipType,

    #[serde(rename = "showPic")]
    pub show_pic: String,

    #[serde(rename = "rankLevel")]
    pub rank_level: i64,

    #[serde(rename = "mainPropDepotId")]
    pub main_prop_depot_id: i64,

    #[serde(rename = "appendPropDepotId")]
    pub append_prop_depot_id: i64,

    #[serde(rename = "addPropLevels")]
    pub add_prop_levels: Vec<i64>,

    #[serde(rename = "baseConvExp")]
    pub base_conv_exp: i64,

    #[serde(rename = "maxLevel")]
    pub max_level: i64,

    #[serde(rename = "destroyReturnMaterial")]
    pub destroy_return_material: Vec<i64>,

    #[serde(rename = "destroyReturnMaterialCount")]
    pub destroy_return_material_count: Vec<i64>,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "itemType")]
    pub item_type: ItemType,

    #[serde(rename = "weight")]
    pub weight: i64,

    #[serde(rename = "rank")]
    pub rank: i64,

    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,

    #[serde(rename = "appendPropNum")]
    pub append_prop_num: Option<i64>,

    #[serde(rename = "setId")]
    pub set_id: Option<i64>,

    #[serde(rename = "storyId")]
    pub story_id: Option<i64>,

    #[serde(rename = "destroyRule")]
    pub destroy_rule: Option<DestroyRule>,

    #[serde(rename = "dropable")]
    pub dropable: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DestroyRule {
    #[serde(rename = "DESTROY_RETURN_MATERIAL")]
    DestroyReturnMaterial,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EquipType {
    #[serde(rename = "EQUIP_BRACER")]
    EquipBracer,

    #[serde(rename = "EQUIP_DRESS")]
    EquipDress,

    #[serde(rename = "EQUIP_NECKLACE")]
    EquipNecklace,

    #[serde(rename = "EQUIP_RING")]
    EquipRing,

    #[serde(rename = "EQUIP_SHOES")]
    EquipShoes,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "ITEM_RELIQUARY")]
    ItemReliquary,
}

pub fn load() -> Result<ReliquaryExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ReliquaryExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
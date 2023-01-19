// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type HomeWorldFurnitureExcelConfigData = Vec<HomeWorldFurnitureExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeWorldFurnitureExcelConfigDatum {
    #[serde(rename = "furnitureGadgetID")]
    pub furniture_gadget_id: Vec<i64>,

    #[serde(rename = "furnType")]
    pub furn_type: Vec<i64>,

    #[serde(rename = "surfaceType")]
    pub surface_type: Option<SurfaceType>,

    #[serde(rename = "gridStyle")]
    pub grid_style: Option<i64>,

    #[serde(rename = "comfort")]
    pub comfort: Option<i64>,

    #[serde(rename = "stackLimit")]
    pub stack_limit: Option<i64>,

    #[serde(rename = "cost")]
    pub cost: Option<i64>,

    #[serde(rename = "JCNHMPNPAGK")]
    pub jcnhmpnpagk: Option<i64>,

    #[serde(rename = "itemIcon")]
    pub item_icon: String,

    #[serde(rename = "effectIcon")]
    pub effect_icon: EffectIcon,

    #[serde(rename = "NNMAFILLDKD")]
    pub nnmafilldkd: Option<f64>,

    #[serde(rename = "rankLevel")]
    pub rank_level: i64,

    #[serde(rename = "jsonName")]
    pub json_name: JsonName,

    #[serde(rename = "IMMMCMKDBPK")]
    pub immmcmkdbpk: Vec<i64>,

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

    #[serde(rename = "rank")]
    pub rank: Option<i64>,

    #[serde(rename = "KPDGOCHDLHI")]
    pub kpdgochdlhi: Option<Kpdgochdlhi>,

    #[serde(rename = "isUnique")]
    pub is_unique: Option<i64>,

    #[serde(rename = "isSpecialFurniture")]
    pub is_special_furniture: Option<i64>,

    #[serde(rename = "height")]
    pub height: Option<f64>,

    #[serde(rename = "CFCONCFFOFL")]
    pub cfconcffofl: Option<i64>,

    #[serde(rename = "GDNDDAJPJNJ")]
    pub gdnddajpjnj: Option<i64>,

    #[serde(rename = "GCIKBHNEAKP")]
    pub gcikbhneakp: Option<i64>,

    #[serde(rename = "PEFFHINKOCE")]
    pub peffhinkoce: Option<i64>,

    #[serde(rename = "roomSceneID")]
    pub room_scene_id: Option<i64>,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: Option<i64>,

    #[serde(rename = "GEPNMEPLEMN")]
    pub gepnmeplemn: Option<String>,

    #[serde(rename = "MHBBPCHKCFP")]
    pub mhbbpchkcfp: Option<i64>,

    #[serde(rename = "furnitureNameTextMapHash")]
    pub furniture_name_text_map_hash: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EffectIcon {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_Buff_Furniture_MarkTransPoint")]
    UiBuffFurnitureMarkTransPoint,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "ITEM_FURNITURE")]
    ItemFurniture,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JsonName {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "HomeworldGroup_0001")]
    HomeworldGroup0001,

    #[serde(rename = "HomeworldGroup_0002")]
    HomeworldGroup0002,

    #[serde(rename = "HomeworldGroup_0003")]
    HomeworldGroup0003,

    #[serde(rename = "HomeworldGroup_0004")]
    HomeworldGroup0004,

    #[serde(rename = "HomeworldGroup_0005")]
    HomeworldGroup0005,

    #[serde(rename = "HomeworldGroup_0006")]
    HomeworldGroup0006,

    #[serde(rename = "HomeworldGroup_0007")]
    HomeworldGroup0007,

    #[serde(rename = "HomeworldGroup_0008")]
    HomeworldGroup0008,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Kpdgochdlhi {
    #[serde(rename = "Apartment")]
    Apartment,

    #[serde(rename = "BlockDependent")]
    BlockDependent,

    #[serde(rename = "ChangeBgmFurniture")]
    ChangeBgmFurniture,

    #[serde(rename = "CoopPictureFrame")]
    CoopPictureFrame,

    #[serde(rename = "CustomBaseFurnitrue")]
    CustomBaseFurnitrue,

    #[serde(rename = "CustomNodeFurnitrue")]
    CustomNodeFurnitrue,

    #[serde(rename = "FarmField")]
    FarmField,

    #[serde(rename = "Fish")]
    Fish,

    #[serde(rename = "Fishpond")]
    Fishpond,

    #[serde(rename = "Fishtank")]
    Fishtank,

    #[serde(rename = "FurnitureSuite")]
    FurnitureSuite,

    #[serde(rename = "GroupFurnitrue")]
    GroupFurnitrue,

    #[serde(rename = "NPC")]
    Npc,

    #[serde(rename = "Paimon")]
    Paimon,

    #[serde(rename = "ServerGadget")]
    ServerGadget,

    #[serde(rename = "TeleportPoint")]
    TeleportPoint,

    #[serde(rename = "VirtualFurnitrue")]
    VirtualFurnitrue,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SurfaceType {
    #[serde(rename = "Animal")]
    Animal,

    #[serde(rename = "Apartment")]
    Apartment,

    #[serde(rename = "Carpet")]
    Carpet,

    #[serde(rename = "Ceil")]
    Ceil,

    #[serde(rename = "Chandelier")]
    Chandelier,

    #[serde(rename = "Door")]
    Door,

    #[serde(rename = "Floor")]
    Floor,

    #[serde(rename = "FurnitureSuite")]
    FurnitureSuite,

    #[serde(rename = "LegoRockery")]
    LegoRockery,

    #[serde(rename = "NPC")]
    Npc,

    #[serde(rename = "Road")]
    Road,

    #[serde(rename = "StackObjPlane")]
    StackObjPlane,

    #[serde(rename = "Stair")]
    Stair,

    #[serde(rename = "Wall")]
    Wall,

    #[serde(rename = "WallBody")]
    WallBody,
}

pub fn load() -> Result<HomeWorldFurnitureExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HomeWorldFurnitureExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}

// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type GcgChooseExcelConfigData = Vec<GcgChooseExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GcgChooseExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "cardType")]
    pub card_type: CardType,

    #[serde(rename = "KBGLJJMDLNJ")]
    pub kbgljjmdlnj: Kbgljjmdlnj,

    #[serde(rename = "MGLOIGHBILB")]
    pub mgloighbilb: Vec<Mgloighbilb>,

    #[serde(rename = "HNCKFLBAKAD")]
    pub hnckflbakad: Vec<Hnckflbakad>,

    #[serde(rename = "condList")]
    pub cond_list: Vec<Mfbhlcbpoie>,

    #[serde(rename = "MFBHLCBPOIE")]
    pub mfbhlcbpoie: Vec<Mfbhlcbpoie>,

    #[serde(rename = "BFBFDLKMADP")]
    pub bfbfdlkmadp: Vec<Bfbfdlkmadp>,

    #[serde(rename = "CANLKLLAFEA")]
    pub canlkllafea: i64,

    #[serde(rename = "BOFCMEIOMIN")]
    pub bofcmeiomin: Option<Bofcmeiomin>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bfbfdlkmadp {
    #[serde(rename = "type")]
    pub bfbfdlkmadp_type: Option<BfbfdlkmadpType>,

    #[serde(rename = "DAACPEJHIAN")]
    pub daacpejhian: Option<bool>,

    #[serde(rename = "GBDFPKAKMMJ")]
    pub gbdfpkakmmj: Option<Hnckflbakad>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mfbhlcbpoie {
    #[serde(rename = "type")]
    pub mfbhlcbpoie_type: Option<MfbhlcbpoieType>,

    #[serde(rename = "value")]
    pub value: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BfbfdlkmadpType {
    #[serde(rename = "CHARACTER_ORDER")]
    CharacterOrder,

    #[serde(rename = "CREATE_ORDER")]
    CreateOrder,

    #[serde(rename = "HAS_TAG")]
    HasTag,

    #[serde(rename = "HP")]
    Hp,

    #[serde(rename = "TOKEN_TO_SHOW")]
    TokenToShow,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Hnckflbakad {
    #[serde(rename = "GCG_TAG_ELEMENT_ANEMO")]
    GcgTagElementAnemo,

    #[serde(rename = "GCG_TAG_ELEMENT_CRYO")]
    GcgTagElementCryo,

    #[serde(rename = "GCG_TAG_ELEMENT_DENDRO")]
    GcgTagElementDendro,

    #[serde(rename = "GCG_TAG_ELEMENT_ELECTRO")]
    GcgTagElementElectro,

    #[serde(rename = "GCG_TAG_ELEMENT_GEO")]
    GcgTagElementGeo,

    #[serde(rename = "GCG_TAG_ELEMENT_HYDRO")]
    GcgTagElementHydro,

    #[serde(rename = "GCG_TAG_ELEMENT_PYRO")]
    GcgTagElementPyro,

    #[serde(rename = "GCG_TAG_NONE")]
    GcgTagNone,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Bofcmeiomin {
    #[serde(rename = "HEALING")]
    Healing,

    #[serde(rename = "RELIC")]
    Relic,

    #[serde(rename = "TALENT")]
    Talent,

    #[serde(rename = "WEAPON")]
    Weapon,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CardType {
    #[serde(rename = "GCG_CARD_CHARACTER")]
    GcgCardCharacter,

    #[serde(rename = "GCG_CARD_SUMMON")]
    GcgCardSummon,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MfbhlcbpoieType {
    #[serde(rename = "ALIVE_CHAR_COUNT")]
    AliveCharCount,

    #[serde(rename = "CARD")]
    Card,

    #[serde(rename = "CHARACTER_HURT_MIN")]
    CharacterHurtMin,

    #[serde(rename = "CHARACTER_NOT_CHARGED_MAX")]
    CharacterNotChargedMax,

    #[serde(rename = "CHARACTER_NOT_CHARGED_MIN")]
    CharacterNotChargedMin,

    #[serde(rename = "HAS_MODIFY_STATE_WITH_TAG")]
    HasModifyStateWithTag,

    #[serde(rename = "IS_ALIVE_CHARACTER")]
    IsAliveCharacter,

    #[serde(rename = "NOT_HAS_MODIFY_STATE")]
    NotHasModifyState,

    #[serde(rename = "NOT_HAS_MODIFY_STATE_WITH_TAG")]
    NotHasModifyStateWithTag,

    #[serde(rename = "ONSTAGE")]
    Onstage,

    #[serde(rename = "SAME_WEAPON_TYPE_CHAR_COUNT")]
    SameWeaponTypeCharCount,

    #[serde(rename = "SAME_WEAPON_TYPE_WITH")]
    SameWeaponTypeWith,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Kbgljjmdlnj {
    #[serde(rename = "ENEMY")]
    Enemy,

    #[serde(rename = "FRIENDLY")]
    Friendly,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Mgloighbilb {
    #[serde(rename = "GCG_TAG_NONE")]
    GcgTagNone,

    #[serde(rename = "GCG_TAG_WEAPON_BOW")]
    GcgTagWeaponBow,

    #[serde(rename = "GCG_TAG_WEAPON_CATALYST")]
    GcgTagWeaponCatalyst,

    #[serde(rename = "GCG_TAG_WEAPON_CLAYMORE")]
    GcgTagWeaponClaymore,

    #[serde(rename = "GCG_TAG_WEAPON_POLE")]
    GcgTagWeaponPole,

    #[serde(rename = "GCG_TAG_WEAPON_SWORD")]
    GcgTagWeaponSword,
}

pub fn load() -> Result<GcgChooseExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "GCGChooseExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
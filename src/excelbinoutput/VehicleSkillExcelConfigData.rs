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

pub type VehicleSkillExcelConfigData = Vec<VehicleSkillExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct VehicleSkillExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "NIBHCGIHAOF")]
    pub nibhcgihaof: i64,

    #[serde(rename = "JHLHNFNBMAK")]
    pub jhlhnfnbmak: i64,

    #[serde(rename = "abilityName")]
    pub ability_name: String,

    #[serde(rename = "skillIcon")]
    pub skill_icon: String,

    #[serde(rename = "HJKGCNIEHML")]
    pub hjkgcniehml: f64,

    #[serde(rename = "ANMBEJDOCPB")]
    pub anmbejdocpb: i64,

    #[serde(rename = "lockShape")]
    pub lock_shape: String,

    #[serde(rename = "lockWeightParams")]
    pub lock_weight_params: Vec<f64>,

    #[serde(rename = "globalValueKey")]
    pub global_value_key: String,

    #[serde(rename = "OBIBGHGPELD")]
    pub obibghgpeld: Option<i64>,

    #[serde(rename = "NNIEEBPHLEJ")]
    pub nnieebphlej: Option<i64>,
}

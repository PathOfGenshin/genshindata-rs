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

pub type ActivityCrystalLinkEffectBuffExcelConfigData =
    Vec<ActivityCrystalLinkEffectBuffExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityCrystalLinkEffectBuffExcelConfigDatum {
    #[serde(rename = "buffId")]
    pub buff_id: i64,

    #[serde(rename = "GPDAEIMDCBF")]
    pub gpdaeimdcbf: String,

    #[serde(rename = "abilityName")]
    pub ability_name: String,

    #[serde(rename = "GPJHNOBPDJI")]
    pub gpjhnobpdji: i64,

    #[serde(rename = "JICNBGIOOHI")]
    pub jicnbgioohi: i64,

    #[serde(rename = "NBLOCEJHFCN")]
    pub nblocejhfcn: i64,

    #[serde(rename = "HPIKALMBHLL")]
    pub hpikalmbhll: i64,

    #[serde(rename = "FFDCLDPEEML")]
    pub ffdcldpeeml: Vec<String>,
}

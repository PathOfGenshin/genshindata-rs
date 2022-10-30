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

pub type OfferingOpenStateConfigData = Vec<OfferingOpenStateConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct OfferingOpenStateConfigDatum {
    #[serde(rename = "offeringId")]
    pub offering_id: i64,

    #[serde(rename = "openState")]
    pub open_state: String,

    #[serde(rename = "itemLimit")]
    pub item_limit: String,

    #[serde(rename = "GLHGMGDLHEI")]
    pub glhgmgdlhei: String,

    #[serde(rename = "PCKMJCOKEOD")]
    pub pckmjcokeod: Option<bool>,

    #[serde(rename = "GDFHKAGNAEB")]
    pub gdfhkagnaeb: Option<bool>,

    #[serde(rename = "activityId")]
    pub activity_id: Option<i64>,
}

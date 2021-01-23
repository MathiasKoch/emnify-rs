/*
 * EMnify Rest API
 *
 * Rest API resources of the EMnify System.
 *
 * The version of the OpenAPI document: 1.2.26
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineObject {
    #[serde(rename = "type")]
    pub _type: serde_json::Value,
    #[serde(rename = "password")]
    pub password: String,
}

impl InlineObject {
    pub fn new(_type: serde_json::Value, password: String) -> InlineObject {
        InlineObject {
            _type,
            password,
        }
    }
}


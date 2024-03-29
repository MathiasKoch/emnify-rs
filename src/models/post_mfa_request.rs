/*
 * EMnify Rest API
 *
 * Rest API resources of the EMnify System.
 *
 * The version of the OpenAPI document: 1.3
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PostMfaRequest {
    #[serde(rename = "type")]
    pub r#type: serde_json::Value,
    #[serde(rename = "password")]
    pub password: String,
}

impl PostMfaRequest {
    pub fn new(r#type: serde_json::Value, password: String) -> PostMfaRequest {
        PostMfaRequest { r#type, password }
    }
}

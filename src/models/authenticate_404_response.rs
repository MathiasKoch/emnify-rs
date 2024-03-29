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
pub struct Authenticate404Response {
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "error_token", skip_serializing_if = "Option::is_none")]
    pub error_token: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Authenticate404Response {
    pub fn new() -> Authenticate404Response {
        Authenticate404Response {
            error_code: None,
            error_token: None,
            message: None,
        }
    }
}

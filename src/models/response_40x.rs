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
pub struct Response40x {
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<f32>,
    #[serde(rename = "error_token", skip_serializing_if = "Option::is_none")]
    pub error_token: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl Response40x {
    pub fn new() -> Response40x {
        Response40x {
            error_code: None,
            error_token: None,
            message: None,
        }
    }
}


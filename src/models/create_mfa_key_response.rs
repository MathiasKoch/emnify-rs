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
pub struct CreateMfaKeyResponse {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f32>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::CreateMfaKeyResponseStatus>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::CreateMfaKeyResponseStatus>,
    #[serde(rename = "secret_key", skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    #[serde(rename = "otpauth", skip_serializing_if = "Option::is_none")]
    pub otpauth: Option<String>,
    #[serde(rename = "creation_date", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
}

impl CreateMfaKeyResponse {
    pub fn new() -> CreateMfaKeyResponse {
        CreateMfaKeyResponse {
            id: None,
            status: None,
            _type: None,
            secret_key: None,
            otpauth: None,
            creation_date: None,
        }
    }
}



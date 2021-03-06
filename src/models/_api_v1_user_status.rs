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
pub struct ApiV1UserStatus {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ApiV1UserStatus {
    pub fn new() -> ApiV1UserStatus {
        ApiV1UserStatus {
            id: None,
            description: None,
        }
    }
}



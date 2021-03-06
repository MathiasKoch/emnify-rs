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
pub struct ApiV1EndpointStatus {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl ApiV1EndpointStatus {
    pub fn new() -> ApiV1EndpointStatus {
        ApiV1EndpointStatus {
            id: None,
        }
    }
}



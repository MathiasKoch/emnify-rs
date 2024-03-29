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
pub struct GetDataStreamerStatuses200ResponseInner {
    /// Enum item identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f32>,
    /// Enum item description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl GetDataStreamerStatuses200ResponseInner {
    pub fn new() -> GetDataStreamerStatuses200ResponseInner {
        GetDataStreamerStatuses200ResponseInner {
            id: None,
            description: None,
        }
    }
}

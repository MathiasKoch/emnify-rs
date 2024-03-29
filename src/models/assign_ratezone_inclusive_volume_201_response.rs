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
pub struct AssignRatezoneInclusiveVolume201Response {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "pooled", skip_serializing_if = "Option::is_none")]
    pub pooled: Option<bool>,
}

impl AssignRatezoneInclusiveVolume201Response {
    pub fn new() -> AssignRatezoneInclusiveVolume201Response {
        AssignRatezoneInclusiveVolume201Response {
            id: None,
            start_date: None,
            end_date: None,
            pooled: None,
        }
    }
}

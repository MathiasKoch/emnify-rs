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
pub struct ActivateBatch {
    #[serde(rename = "sim_status", skip_serializing_if = "Option::is_none")]
    pub sim_status: Option<Box<crate::models::Status>>,
}

impl ActivateBatch {
    pub fn new() -> ActivateBatch {
        ActivateBatch { sim_status: None }
    }
}

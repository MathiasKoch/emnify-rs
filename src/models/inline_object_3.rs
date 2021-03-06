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
pub struct InlineObject3 {
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "pooled", skip_serializing_if = "Option::is_none")]
    pub pooled: Option<bool>,
}

impl InlineObject3 {
    pub fn new() -> InlineObject3 {
        InlineObject3 {
            start_date: None,
            end_date: None,
            pooled: None,
        }
    }
}



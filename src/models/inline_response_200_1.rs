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
pub struct InlineResponse2001 {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "country_name", skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f32>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<crate::models::ApiV1OperatorCountry>,
}

impl InlineResponse2001 {
    pub fn new() -> InlineResponse2001 {
        InlineResponse2001 {
            name: None,
            country_name: None,
            id: None,
            country: None,
        }
    }
}


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
pub struct ApiV1DataStreamApiCallback {
    #[serde(rename = "organisation_id", skip_serializing_if = "Option::is_none")]
    pub organisation_id: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "purpose", skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "route", skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(rename = "reqParams", skip_serializing_if = "Option::is_none")]
    pub req_params: Option<String>,
    #[serde(rename = "fromServer", skip_serializing_if = "Option::is_none")]
    pub from_server: Option<bool>,
    #[serde(rename = "parentResource", skip_serializing_if = "Option::is_none")]
    pub parent_resource: Option<String>,
    #[serde(rename = "restangularCollection", skip_serializing_if = "Option::is_none")]
    pub restangular_collection: Option<bool>,
}

impl ApiV1DataStreamApiCallback {
    pub fn new() -> ApiV1DataStreamApiCallback {
        ApiV1DataStreamApiCallback {
            organisation_id: None,
            url: None,
            created: None,
            purpose: None,
            id: None,
            route: None,
            req_params: None,
            from_server: None,
            parent_resource: None,
            restangular_collection: None,
        }
    }
}


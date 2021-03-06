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
pub struct InlineResponse200 {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "organisation_type_id", skip_serializing_if = "Option::is_none")]
    pub organisation_type_id: Option<String>,
    #[serde(rename = "country_id", skip_serializing_if = "Option::is_none")]
    pub country_id: Option<String>,
    #[serde(rename = "organisation_status_id", skip_serializing_if = "Option::is_none")]
    pub organisation_status_id: Option<String>,
    #[serde(rename = "ext_reference", skip_serializing_if = "Option::is_none")]
    pub ext_reference: Option<String>,
    #[serde(rename = "monthly_cost_limit", skip_serializing_if = "Option::is_none")]
    pub monthly_cost_limit: Option<String>,
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<i32>,
    #[serde(rename = "organisation_class_id", skip_serializing_if = "Option::is_none")]
    pub organisation_class_id: Option<i32>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "verification_type_id", skip_serializing_if = "Option::is_none")]
    pub verification_type_id: Option<String>,
    #[serde(rename = "verification", skip_serializing_if = "Option::is_none")]
    pub verification: Option<String>,
    #[serde(rename = "brand_id", skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<String>,
    #[serde(rename = "default_sms_routing_id", skip_serializing_if = "Option::is_none")]
    pub default_sms_routing_id: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl InlineResponse200 {
    pub fn new() -> InlineResponse200 {
        InlineResponse200 {
            name: None,
            organisation_type_id: None,
            country_id: None,
            organisation_status_id: None,
            ext_reference: None,
            monthly_cost_limit: None,
            currency_id: None,
            organisation_class_id: None,
            created: None,
            verification_type_id: None,
            verification: None,
            brand_id: None,
            default_sms_routing_id: None,
            id: None,
        }
    }
}



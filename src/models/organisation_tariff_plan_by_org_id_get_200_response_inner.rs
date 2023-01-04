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
pub struct OrganisationTariffPlanByOrgIdGet200ResponseInner {
    /// ID of the tariff plan assignment
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f32>,
    #[serde(rename = "organisation_id", skip_serializing_if = "Option::is_none")]
    pub organisation_id: Option<String>,
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "expiry_date", skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(rename = "sim_activated_rate", skip_serializing_if = "Option::is_none")]
    pub sim_activated_rate: Option<f32>,
    #[serde(
        rename = "sim_activated_idle_rate",
        skip_serializing_if = "Option::is_none"
    )]
    pub sim_activated_idle_rate: Option<f32>,
    #[serde(rename = "sim_suspended_rate", skip_serializing_if = "Option::is_none")]
    pub sim_suspended_rate: Option<f32>,
    #[serde(
        rename = "sim_suspended_active_rate",
        skip_serializing_if = "Option::is_none"
    )]
    pub sim_suspended_active_rate: Option<f32>,
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// Custom federation_allowed configuration for the organisation. If set to null, the default configuration from the tariff plan will be applied.
    #[serde(rename = "federation_allowed", skip_serializing_if = "Option::is_none")]
    pub federation_allowed: Option<bool>,
    #[serde(rename = "service_level", skip_serializing_if = "Option::is_none")]
    pub service_level:
        Option<Box<crate::models::OrganisationTariffPlanByOrgIdGet200ResponseInnerServiceLevel>>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "applied_rate", skip_serializing_if = "Option::is_none")]
    pub applied_rate: Option<f32>,
    #[serde(rename = "price_model", skip_serializing_if = "Option::is_none")]
    pub price_model: Option<String>,
    #[serde(rename = "tariff_plan", skip_serializing_if = "Option::is_none")]
    pub tariff_plan:
        Option<Box<crate::models::OrganisationTariffPlanByOrgIdGet200ResponseInnerTariffPlan>>,
}

impl OrganisationTariffPlanByOrgIdGet200ResponseInner {
    pub fn new() -> OrganisationTariffPlanByOrgIdGet200ResponseInner {
        OrganisationTariffPlanByOrgIdGet200ResponseInner {
            id: None,
            organisation_id: None,
            start_date: None,
            expiry_date: None,
            sim_activated_rate: None,
            sim_activated_idle_rate: None,
            sim_suspended_rate: None,
            sim_suspended_active_rate: None,
            currency_id: None,
            federation_allowed: None,
            service_level: None,
            active: None,
            applied_rate: None,
            price_model: None,
            tariff_plan: None,
        }
    }
}
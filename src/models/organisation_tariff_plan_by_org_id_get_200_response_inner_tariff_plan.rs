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
pub struct OrganisationTariffPlanByOrgIdGet200ResponseInnerTariffPlan {
    #[serde(rename = "organisation_id", skip_serializing_if = "Option::is_none")]
    pub organisation_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "tariff_plan_type_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub tariff_plan_type_id: Option<String>,
    #[serde(
        rename = "min_tariff_plan_runtime_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_tariff_plan_runtime_id: Option<String>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(
        rename = "tariff_plan_runtime_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub tariff_plan_runtime_id: Option<serde_json::Value>,
    #[serde(
        rename = "owner_organisation_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub owner_organisation_id: Option<String>,
    #[serde(
        rename = "public_for_child_organisations",
        skip_serializing_if = "Option::is_none"
    )]
    pub public_for_child_organisations: Option<bool>,
    #[serde(rename = "creation_date", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "activation_date", skip_serializing_if = "Option::is_none")]
    pub activation_date: Option<String>,
    #[serde(rename = "deprecation_date", skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<String>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<serde_json::Value>,
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
    #[serde(rename = "yearly_rate", skip_serializing_if = "Option::is_none")]
    pub yearly_rate: Option<f32>,
    #[serde(rename = "sim_activation_fee", skip_serializing_if = "Option::is_none")]
    pub sim_activation_fee: Option<f32>,
    #[serde(rename = "max_active_sims", skip_serializing_if = "Option::is_none")]
    pub max_active_sims: Option<f32>,
    #[serde(
        rename = "add_active_sims_rate",
        skip_serializing_if = "Option::is_none"
    )]
    pub add_active_sims_rate: Option<f32>,
    #[serde(
        rename = "add_active_sim_batch",
        skip_serializing_if = "Option::is_none"
    )]
    pub add_active_sim_batch: Option<f32>,
    #[serde(
        rename = "max_service_profiles",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_service_profiles: Option<f32>,
    #[serde(
        rename = "max_tariff_profiles",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_tariff_profiles: Option<f32>,
    #[serde(rename = "max_users", skip_serializing_if = "Option::is_none")]
    pub max_users: Option<f32>,
    #[serde(rename = "minimum_runtime", skip_serializing_if = "Option::is_none")]
    pub minimum_runtime: Option<f32>,
    #[serde(rename = "api_access_allowed", skip_serializing_if = "Option::is_none")]
    pub api_access_allowed: Option<f32>,
    #[serde(rename = "vpn_access", skip_serializing_if = "Option::is_none")]
    pub vpn_access: Option<f32>,
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<f32>,
    #[serde(rename = "dtype", skip_serializing_if = "Option::is_none")]
    pub dtype: Option<String>,
    #[serde(rename = "evaluation", skip_serializing_if = "Option::is_none")]
    pub evaluation: Option<bool>,
    #[serde(rename = "expiry_time", skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<f32>,
    #[serde(rename = "billing_disabled", skip_serializing_if = "Option::is_none")]
    pub billing_disabled: Option<bool>,
    #[serde(rename = "used_count", skip_serializing_if = "Option::is_none")]
    pub used_count: Option<f32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f32>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<
        Box<crate::models::OrganisationTariffPlanByOrgIdGet200ResponseInnerTariffPlanStatus>,
    >,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<
        Box<crate::models::OrganisationTariffPlanByOrgIdGet200ResponseInnerTariffPlanCurrency>,
    >,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<
        Box<crate::models::OrganisationTariffPlanByOrgIdGet200ResponseInnerTariffPlanVisibility>,
    >,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price:
        Option<Box<crate::models::OrganisationTariffPlanByOrgIdGet200ResponseInnerTariffPlanPrice>>,
    #[serde(rename = "service_level", skip_serializing_if = "Option::is_none")]
    pub service_level: Option<
        Box<crate::models::OrganisationTariffPlanByOrgIdGet200ResponseInnerTariffPlanServiceLevel>,
    >,
    #[serde(rename = "federation_allowed", skip_serializing_if = "Option::is_none")]
    pub federation_allowed: Option<bool>,
}

impl OrganisationTariffPlanByOrgIdGet200ResponseInnerTariffPlan {
    pub fn new() -> OrganisationTariffPlanByOrgIdGet200ResponseInnerTariffPlan {
        OrganisationTariffPlanByOrgIdGet200ResponseInnerTariffPlan {
            organisation_id: None,
            name: None,
            description: None,
            tariff_plan_type_id: None,
            min_tariff_plan_runtime_id: None,
            level: None,
            tariff_plan_runtime_id: None,
            owner_organisation_id: None,
            public_for_child_organisations: None,
            creation_date: None,
            activation_date: None,
            deprecation_date: None,
            notes: None,
            rate: None,
            yearly_rate: None,
            sim_activation_fee: None,
            max_active_sims: None,
            add_active_sims_rate: None,
            add_active_sim_batch: None,
            max_service_profiles: None,
            max_tariff_profiles: None,
            max_users: None,
            minimum_runtime: None,
            api_access_allowed: None,
            vpn_access: None,
            private: None,
            dtype: None,
            evaluation: None,
            expiry_time: None,
            billing_disabled: None,
            used_count: None,
            id: None,
            status: None,
            currency: None,
            visibility: None,
            price: None,
            service_level: None,
            federation_allowed: None,
        }
    }
}

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
pub struct GetOrganisationActiveTariffPlanResponseTariffPlanPrice {
    #[serde(rename = "sim_activated_rate", skip_serializing_if = "Option::is_none")]
    pub sim_activated_rate: Option<Vec<crate::models::GetOrganisationActiveTariffPlanResponseTariffPlanPriceSimActivatedRateInner>>,
}

impl GetOrganisationActiveTariffPlanResponseTariffPlanPrice {
    pub fn new() -> GetOrganisationActiveTariffPlanResponseTariffPlanPrice {
        GetOrganisationActiveTariffPlanResponseTariffPlanPrice {
            sim_activated_rate: None,
        }
    }
}

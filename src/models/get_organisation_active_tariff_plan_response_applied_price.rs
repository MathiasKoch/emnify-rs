/*
 * EMnify Rest API
 *
 * Rest API resources of the EMnify System.
 *
 * The version of the OpenAPI document: 1.3
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetOrganisationActiveTariffPlanResponseAppliedPrice : the fees that apply for the current month according to the number of activated SIMs. calculation follows volume pricing at the moment, but may be changed at any time. the resulting price is the sum of each price line times the given volume. if it is a scaled price, the rate object contains a `scale_start` field, otherwise it is fixed.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetOrganisationActiveTariffPlanResponseAppliedPrice {
    #[serde(rename = "sim_activated_rate", skip_serializing_if = "Option::is_none")]
    pub sim_activated_rate: Option<
        Vec<
            crate::models::GetOrganisationActiveTariffPlanResponseAppliedPriceSimActivatedRateInner,
        >,
    >,
}

impl GetOrganisationActiveTariffPlanResponseAppliedPrice {
    /// the fees that apply for the current month according to the number of activated SIMs. calculation follows volume pricing at the moment, but may be changed at any time. the resulting price is the sum of each price line times the given volume. if it is a scaled price, the rate object contains a `scale_start` field, otherwise it is fixed.
    pub fn new() -> GetOrganisationActiveTariffPlanResponseAppliedPrice {
        GetOrganisationActiveTariffPlanResponseAppliedPrice {
            sim_activated_rate: None,
        }
    }
}

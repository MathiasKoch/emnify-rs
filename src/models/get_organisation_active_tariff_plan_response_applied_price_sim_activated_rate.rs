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
pub struct GetOrganisationActiveTariffPlanResponseAppliedPriceSimActivatedRate {
    #[serde(rename = "scale_start", skip_serializing_if = "Option::is_none")]
    pub scale_start: Option<f32>,
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<i32>,
}

impl GetOrganisationActiveTariffPlanResponseAppliedPriceSimActivatedRate {
    pub fn new() -> GetOrganisationActiveTariffPlanResponseAppliedPriceSimActivatedRate {
        GetOrganisationActiveTariffPlanResponseAppliedPriceSimActivatedRate {
            scale_start: None,
            rate: None,
            volume: None,
        }
    }
}



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
pub struct TariffPlanFixedPrice {
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
}

impl TariffPlanFixedPrice {
    pub fn new() -> TariffPlanFixedPrice {
        TariffPlanFixedPrice { rate: None }
    }
}
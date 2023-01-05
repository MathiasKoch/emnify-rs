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
pub struct OrganisationDailyTrafficObject {
    /// Total cost
    #[serde(rename = "cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<String>,
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Box<crate::models::Currency>>,
    #[serde(rename = "traffic_type_id", skip_serializing_if = "Option::is_none")]
    pub traffic_type_id: Option<String>,
    #[serde(rename = "traffic_type", skip_serializing_if = "Option::is_none")]
    pub traffic_type: Option<Box<crate::models::OrganisationTrafficStatisticsObjectTrafficType>>,
    /// Total consumption (`volume_rx` + `volume_tx`)
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    /// * For traffic type `5` (`Data`): Downloaded data * For traffic type `6` (`SMS`): SMS MT
    #[serde(rename = "volume_rx", skip_serializing_if = "Option::is_none")]
    pub volume_rx: Option<String>,
    /// * For traffic type `5` (`Data`): Uploaded data * For traffic type `6` (`SMS`): SMS MO
    #[serde(rename = "volume_tx", skip_serializing_if = "Option::is_none")]
    pub volume_tx: Option<String>,
}

impl OrganisationDailyTrafficObject {
    pub fn new() -> OrganisationDailyTrafficObject {
        OrganisationDailyTrafficObject {
            cost: None,
            currency_id: None,
            currency: None,
            traffic_type_id: None,
            traffic_type: None,
            volume: None,
            volume_rx: None,
            volume_tx: None,
        }
    }
}

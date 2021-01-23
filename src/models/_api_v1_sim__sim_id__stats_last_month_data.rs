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
pub struct ApiV1SimSimIdStatsLastMonthData {
    #[serde(rename = "sim_id", skip_serializing_if = "Option::is_none")]
    pub sim_id: Option<f32>,
    #[serde(rename = "month", skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<f32>,
    #[serde(rename = "volume_tx", skip_serializing_if = "Option::is_none")]
    pub volume_tx: Option<f32>,
    #[serde(rename = "volume_rx", skip_serializing_if = "Option::is_none")]
    pub volume_rx: Option<f32>,
    #[serde(rename = "traffic_type_id", skip_serializing_if = "Option::is_none")]
    pub traffic_type_id: Option<f32>,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<f32>,
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<f32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f32>,
    #[serde(rename = "traffic_type", skip_serializing_if = "Option::is_none")]
    pub traffic_type: Option<crate::models::ApiV1SimSimIdStatsLastMonthDataTrafficType>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::models::Currency>,
}

impl ApiV1SimSimIdStatsLastMonthData {
    pub fn new() -> ApiV1SimSimIdStatsLastMonthData {
        ApiV1SimSimIdStatsLastMonthData {
            sim_id: None,
            month: None,
            volume: None,
            volume_tx: None,
            volume_rx: None,
            traffic_type_id: None,
            last_updated: None,
            cost: None,
            currency_id: None,
            id: None,
            traffic_type: None,
            currency: None,
        }
    }
}


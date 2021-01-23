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
pub struct ResponseSchemaForSimStatistics {
    #[serde(rename = "last_month")]
    pub last_month: crate::models::ApiV1SimSimIdStatsLastMonth,
    #[serde(rename = "current_month")]
    pub current_month: crate::models::ApiV1SimSimIdStatsLastMonth,
    #[serde(rename = "last_hour")]
    pub last_hour: crate::models::ApiV1SimSimIdStatsLastMonth,
}

impl ResponseSchemaForSimStatistics {
    pub fn new(last_month: crate::models::ApiV1SimSimIdStatsLastMonth, current_month: crate::models::ApiV1SimSimIdStatsLastMonth, last_hour: crate::models::ApiV1SimSimIdStatsLastMonth) -> ResponseSchemaForSimStatistics {
        ResponseSchemaForSimStatistics {
            last_month,
            current_month,
            last_hour,
        }
    }
}


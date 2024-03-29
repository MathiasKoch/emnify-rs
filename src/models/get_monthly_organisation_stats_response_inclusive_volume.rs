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
pub struct GetMonthlyOrganisationStatsResponseInclusiveVolume {
    /// Total cost of all unpooled inclusive volumes
    #[serde(rename = "unpooled", skip_serializing_if = "Option::is_none")]
    pub unpooled: Option<f32>,
    /// Total cost of all pooled inclusive volumes
    #[serde(rename = "pooled", skip_serializing_if = "Option::is_none")]
    pub pooled: Option<f32>,
}

impl GetMonthlyOrganisationStatsResponseInclusiveVolume {
    pub fn new() -> GetMonthlyOrganisationStatsResponseInclusiveVolume {
        GetMonthlyOrganisationStatsResponseInclusiveVolume {
            unpooled: None,
            pooled: None,
        }
    }
}

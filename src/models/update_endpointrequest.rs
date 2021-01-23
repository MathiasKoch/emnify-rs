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
pub struct UpdateEndpointrequest {
    #[serde(rename = "tags")]
    pub tags: String,
    #[serde(rename = "status")]
    pub status: serde_json::Value,
    #[serde(rename = "service_profile")]
    pub service_profile: serde_json::Value,
    #[serde(rename = "tariff_profile")]
    pub tariff_profile: serde_json::Value,
    #[serde(rename = "ip_address")]
    pub ip_address: String,
    #[serde(rename = "ip_address_space")]
    pub ip_address_space: serde_json::Value,
    #[serde(rename = "sim")]
    pub sim: serde_json::Value,
    #[serde(rename = "imei")]
    pub imei: String,
    #[serde(rename = "imei_lock")]
    pub imei_lock: bool,
}

impl UpdateEndpointrequest {
    pub fn new(tags: String, status: serde_json::Value, service_profile: serde_json::Value, tariff_profile: serde_json::Value, ip_address: String, ip_address_space: serde_json::Value, sim: serde_json::Value, imei: String, imei_lock: bool) -> UpdateEndpointrequest {
        UpdateEndpointrequest {
            tags,
            status,
            service_profile,
            tariff_profile,
            ip_address,
            ip_address_space,
            sim,
            imei,
            imei_lock,
        }
    }
}


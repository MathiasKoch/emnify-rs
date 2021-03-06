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
pub struct UpdateSiMrequest {
    #[serde(rename = "issuer_org")]
    pub issuer_org: serde_json::Value,
    #[serde(rename = "reseller_org")]
    pub reseller_org: serde_json::Value,
    #[serde(rename = "customer_org")]
    pub customer_org: serde_json::Value,
    #[serde(rename = "status")]
    pub status: serde_json::Value,
}

impl UpdateSiMrequest {
    pub fn new(issuer_org: serde_json::Value, reseller_org: serde_json::Value, customer_org: serde_json::Value, status: serde_json::Value) -> UpdateSiMrequest {
        UpdateSiMrequest {
            issuer_org,
            reseller_org,
            customer_org,
            status,
        }
    }
}



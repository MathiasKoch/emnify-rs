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
pub struct ChangePasswordrequest {
    #[serde(rename = "old_password")]
    pub old_password: String,
    #[serde(rename = "new_password")]
    pub new_password: String,
}

impl ChangePasswordrequest {
    pub fn new(old_password: String, new_password: String) -> ChangePasswordrequest {
        ChangePasswordrequest {
            old_password,
            new_password,
        }
    }
}

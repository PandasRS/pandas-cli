use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct {{ModuleName}} {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
{{params_struct}}
}

use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Create{{ModuleName}}Dto {
{{params_struct}}
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Update{{ModuleName}}Dto {
{{params_struct_optional}}
}

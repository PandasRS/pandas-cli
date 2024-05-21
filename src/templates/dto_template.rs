use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Create{{ModuleName}}Dto {
    pub name: String,
    pub age: i32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Update{{ModuleName}}Dto {
    pub name: Option<String>,
    pub age: Option<i32>,
}

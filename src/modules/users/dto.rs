use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CreateUsersDto {
    pub name: String,
    pub age: i32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UpdateUsersDto {
    pub name: Option<String>,
    pub age: Option<i32>,
}

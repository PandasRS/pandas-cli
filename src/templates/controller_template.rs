use rocket::{State, serde::json::Json};
use rocket_okapi::openapi;
use mongodb::Database;

use crate::modules::{{module_name}}::dto::{Create{{ModuleName}}Dto, Update{{ModuleName}}Dto};
use crate::modules::{{module_name}}::schema::{{ModuleName}};
use crate::modules::{{module_name}}::service;
use crate::modules::{{module_name}}::repository::Mongo{{ModuleName}}Repository;

#[openapi(tag = "{{ModuleName}}")]
#[post("/{{module_name}}", format = "json", data = "<dto>")]
pub async fn create_{{module_name}}(dto: Json<Create{{ModuleName}}Dto>, db: &State<Database>) -> Json<{{ModuleName}}> {
    let mut repo = Mongo{{ModuleName}}Repository::new(db.inner().clone());
    service::create_{{module_name}}(&mut repo, dto.into_inner()).await
}

#[openapi(tag = "{{ModuleName}}")]
#[get("/{{module_name}}")]
pub async fn get_{{module_name}}(db: &State<Database>) -> Json<Vec<{{ModuleName}}>> {
    let mut repo = Mongo{{ModuleName}}Repository::new(db.inner().clone());
    service::get_{{module_name}}(&mut repo).await
}

#[openapi(tag = "{{ModuleName}}")]
#[get("/{{module_name}}/<id>")]
pub async fn get_{{module_name}}_by_id(id: String, db: &State<Database>) -> Option<Json<{{ModuleName}}>> {
    let mut repo = Mongo{{ModuleName}}Repository::new(db.inner().clone());
    service::get_{{module_name}}_by_id(&mut repo, &id).await
}

#[openapi(tag = "{{ModuleName}}")]
#[put("/{{module_name}}/<id>", format = "json", data = "<dto>")]
pub async fn update_{{module_name}}(id: String, dto: Json<Update{{ModuleName}}Dto>, db: &State<Database>) -> Option<Json<{{ModuleName}}>> {
    let mut repo = Mongo{{ModuleName}}Repository::new(db.inner().clone());
    service::update_{{module_name}}(&mut repo, &id, dto.into_inner()).await
}

#[openapi(tag = "{{ModuleName}}")]
#[delete("/{{module_name}}/<id>")]
pub async fn delete_{{module_name}}(id: String, db: &State<Database>) -> Json<bool> {
    let mut repo = Mongo{{ModuleName}}Repository::new(db.inner().clone());
    Json(service::delete_{{module_name}}(&mut repo, &id).await)
}

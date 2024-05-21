use rocket::{State, serde::json::Json};
use rocket_okapi::openapi;
use mongodb::Database;

use crate::modules::users::dto::{CreateUsersDto, UpdateUsersDto};
use crate::modules::users::schema::Users;
use crate::modules::users::service;
use crate::modules::users::repository::MongoUsersRepository;

#[openapi(tag = "Users")]
#[post("/users", format = "json", data = "<dto>")]
pub async fn create_users(dto: Json<CreateUsersDto>, db: &State<Database>) -> Json<Users> {
    let mut repo = MongoUsersRepository::new(db.inner().clone());
    service::create_users(&mut repo, dto.into_inner()).await
}

#[openapi(tag = "Users")]
#[get("/users")]
pub async fn get_users(db: &State<Database>) -> Json<Vec<Users>> {
    let mut repo = MongoUsersRepository::new(db.inner().clone());
    service::get_users(&mut repo).await
}

#[openapi(tag = "Users")]
#[get("/users/<id>")]
pub async fn get_users_by_id(id: String, db: &State<Database>) -> Option<Json<Users>> {
    let mut repo = MongoUsersRepository::new(db.inner().clone());
    service::get_users_by_id(&mut repo, &id).await
}

#[openapi(tag = "Users")]
#[put("/users/<id>", format = "json", data = "<dto>")]
pub async fn update_users(id: String, dto: Json<UpdateUsersDto>, db: &State<Database>) -> Option<Json<Users>> {
    let mut repo = MongoUsersRepository::new(db.inner().clone());
    service::update_users(&mut repo, &id, dto.into_inner()).await
}

#[openapi(tag = "Users")]
#[delete("/users/<id>")]
pub async fn delete_users(id: String, db: &State<Database>) -> Json<bool> {
    let mut repo = MongoUsersRepository::new(db.inner().clone());
    Json(service::delete_users(&mut repo, &id).await)
}

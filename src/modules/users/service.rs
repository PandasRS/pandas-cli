use rocket::serde::json::Json;
use crate::modules::users::dto::{CreateUsersDto, UpdateUsersDto};
use crate::modules::users::repository::UsersRepository;
use crate::modules::users::schema::Users;

pub async fn create_users<R: UsersRepository>(repo: &mut R, dto: CreateUsersDto) -> Json<Users> {
    let users = Users {
        id: None,
        name: dto.name,
        age: dto.age,
    };
    Json(repo.create_users(users).await)
}

pub async fn get_users<R: UsersRepository>(repo: &mut R) -> Json<Vec<Users>> {
    Json(repo.get_users().await)
}

pub async fn get_users_by_id<R: UsersRepository>(repo: &mut R, id: &String) -> Option<Json<Users>> {
    repo.get_users_by_id(id).await.map(Json)
}

pub async fn update_users<R: UsersRepository>(repo: &mut R, id: &str, dto: UpdateUsersDto) -> Option<Json<Users>> {
    repo.update_users(id, dto).await.map(Json)
}

pub async fn delete_users<R: UsersRepository>(repo: &mut R, id: &str) -> bool {
    repo.delete_users(id).await
}

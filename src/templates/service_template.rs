use rocket::serde::json::Json;
use crate::modules::{{module_name}}::dto::{Create{{ModuleName}}Dto, Update{{ModuleName}}Dto};
use crate::modules::{{module_name}}::repository::{{ModuleName}}Repository;
use crate::modules::{{module_name}}::schema::{{ModuleName}};

pub async fn create_{{module_name}}<R: {{ModuleName}}Repository>(repo: &mut R, dto: Create{{ModuleName}}Dto) -> Json<{{ModuleName}}> {
    let {{module_name}} = {{ModuleName}} {
        id: None,
        name: dto.name,
        age: dto.age,
    };
    Json(repo.create_{{module_name}}({{module_name}}).await)
}

pub async fn get_{{module_name}}<R: {{ModuleName}}Repository>(repo: &mut R) -> Json<Vec<{{ModuleName}}>> {
    Json(repo.get_{{module_name}}().await)
}

pub async fn get_{{module_name}}_by_id<R: {{ModuleName}}Repository>(repo: &mut R, id: &String) -> Option<Json<{{ModuleName}}>> {
    repo.get_{{module_name}}_by_id(id).await.map(Json)
}

pub async fn update_{{module_name}}<R: {{ModuleName}}Repository>(repo: &mut R, id: &str, dto: Update{{ModuleName}}Dto) -> Option<Json<{{ModuleName}}>> {
    repo.update_{{module_name}}(id, dto).await.map(Json)
}

pub async fn delete_{{module_name}}<R: {{ModuleName}}Repository>(repo: &mut R, id: &str) -> bool {
    repo.delete_{{module_name}}(id).await
}

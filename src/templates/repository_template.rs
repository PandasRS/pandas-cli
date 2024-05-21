use mongodb::{bson::{doc, Document, oid::ObjectId}, Collection};
use futures::stream::StreamExt;
use async_trait::async_trait;
use crate::modules::{{module_name}}::schema::{{ModuleName}};
use crate::modules::{{module_name}}::dto::Update{{ModuleName}}Dto;

#[async_trait]
pub trait {{ModuleName}}Repository {
    async fn create_{{module_name}}(&mut self, {{module_name}}: {{ModuleName}}) -> {{ModuleName}};
    async fn get_{{module_name}}(&mut self) -> Vec<{{ModuleName}}>;
    async fn get_{{module_name}}_by_id(&mut self, id: &str) -> Option<{{ModuleName}}>;
    async fn update_{{module_name}}(&mut self, id: &str, dto: Update{{ModuleName}}Dto) -> Option<{{ModuleName}}>;
    async fn delete_{{module_name}}(&mut self, id: &str) -> bool;
}

pub struct Mongo{{ModuleName}}Repository {
    db: mongodb::Database,
}

impl Mongo{{ModuleName}}Repository {
    pub fn new(db: mongodb::Database) -> Self {
        Mongo{{ModuleName}}Repository { db }
    }
}

#[async_trait]
impl {{ModuleName}}Repository for Mongo{{ModuleName}}Repository {
    async fn create_{{module_name}}(&mut self, mut {{module_name}}: {{ModuleName}}) -> {{ModuleName}} {
        let collection = self.db.collection("{{module_name}}s");
        match collection.insert_one({{module_name}}.clone(), None).await {
            Ok(result) => {
                {{module_name}}.id = result.inserted_id.as_object_id().map(|oid| oid.to_hex());
                {{module_name}}
            },
            Err(_) => panic!("Error inserting {{module_name}}"),
        }
    }

    async fn get_{{module_name}}(&mut self) -> Vec<{{ModuleName}}> {
        let collection: Collection<Document> = self.db.collection("{{module_name}}s");
        let mut cursor = collection.find(doc! {}, None).await.unwrap();
        let mut {{module_name}}s: Vec<{{ModuleName}}> = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let mut {{module_name}}: {{ModuleName}} = mongodb::bson::from_document(document.clone()).unwrap();
                    if let Some(oid) = document.get_object_id("_id").ok() {
                        {{module_name}}.id = Some(oid.to_hex());
                    }
                    {{module_name}}s.push({{module_name}});
                },
                Err(_) => panic!("Error getting {{module_name}}"),
            }
        }
        {{module_name}}s
    }

    async fn get_{{module_name}}_by_id(&mut self, id: &str) -> Option<{{ModuleName}}> {
        let collection: Collection<Document> = self.db.collection("{{module_name}}s");
        if let Ok(object_id) = ObjectId::parse_str(id) {
            match collection.find_one(doc! {"_id": object_id}, None).await {
                Ok(document) => {
                    document.map(|doc| {
                        let mut {{module_name}}: {{ModuleName}} = mongodb::bson::from_document(doc).unwrap();
                        {{module_name}}.id = Some(id.to_string());
                        {{module_name}}
                    })
                },
                Err(_) => panic!("Error getting {{module_name}}"),
            }
        } else {
            None
        }
    }

    async fn update_{{module_name}}(&mut self, id: &str, dto: Update{{ModuleName}}Dto) -> Option<{{ModuleName}}> {
        let collection: Collection<Document> = self.db.collection("{{module_name}}s");
        if let Ok(object_id) = ObjectId::parse_str(id) {
            let mut update_doc = doc! {};
            if let Some(name) = dto.name {
                update_doc.insert("name", name);
            }
            if let Some(age) = dto.age {
                update_doc.insert("age", age);
            }
            match collection.update_one(doc! {"_id": object_id}, doc! {"$set": update_doc}, None).await {
                Ok(_) => collection.find_one(doc! {"_id": object_id}, None).await.unwrap().map(|doc| {
                    let mut {{module_name}}: {{ModuleName}} = mongodb::bson::from_document(doc).unwrap();
                    {{module_name}}.id = Some(id.to_string());
                    {{module_name}}
                }),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    async fn delete_{{module_name}}(&mut self, id: &str) -> bool {
        let collection: Collection<Document> = self.db.collection("{{module_name}}s");
        if let Ok(object_id) = ObjectId::parse_str(id) {
            match collection.delete_one(doc! {"_id": object_id}, None).await {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            false
        }
    }
}

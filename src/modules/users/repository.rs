use mongodb::{bson::{doc, Document, oid::ObjectId}, Collection};
use futures::stream::StreamExt;
use async_trait::async_trait;
use crate::modules::users::schema::Users;
use crate::modules::users::dto::UpdateUsersDto;

#[async_trait]
pub trait UsersRepository {
    async fn create_users(&mut self, users: Users) -> Users;
    async fn get_users(&mut self) -> Vec<Users>;
    async fn get_users_by_id(&mut self, id: &str) -> Option<Users>;
    async fn update_users(&mut self, id: &str, dto: UpdateUsersDto) -> Option<Users>;
    async fn delete_users(&mut self, id: &str) -> bool;
}

pub struct MongoUsersRepository {
    db: mongodb::Database,
}

impl MongoUsersRepository {
    pub fn new(db: mongodb::Database) -> Self {
        MongoUsersRepository { db }
    }
}

#[async_trait]
impl UsersRepository for MongoUsersRepository {
    async fn create_users(&mut self, mut users: Users) -> Users {
        let collection = self.db.collection("userss");
        match collection.insert_one(users.clone(), None).await {
            Ok(result) => {
                users.id = result.inserted_id.as_object_id().map(|oid| oid.to_hex());
                users
            },
            Err(_) => panic!("Error inserting users"),
        }
    }

    async fn get_users(&mut self) -> Vec<Users> {
        let collection: Collection<Document> = self.db.collection("userss");
        let mut cursor = collection.find(doc! {}, None).await.unwrap();
        let mut userss: Vec<Users> = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let mut users: Users = mongodb::bson::from_document(document.clone()).unwrap();
                    if let Some(oid) = document.get_object_id("_id").ok() {
                        users.id = Some(oid.to_hex());
                    }
                    userss.push(users);
                },
                Err(_) => panic!("Error getting users"),
            }
        }
        userss
    }

    async fn get_users_by_id(&mut self, id: &str) -> Option<Users> {
        let collection: Collection<Document> = self.db.collection("userss");
        if let Ok(object_id) = ObjectId::parse_str(id) {
            match collection.find_one(doc! {"_id": object_id}, None).await {
                Ok(document) => {
                    document.map(|doc| {
                        let mut users: Users = mongodb::bson::from_document(doc).unwrap();
                        users.id = Some(id.to_string());
                        users
                    })
                },
                Err(_) => panic!("Error getting users"),
            }
        } else {
            None
        }
    }

    async fn update_users(&mut self, id: &str, dto: UpdateUsersDto) -> Option<Users> {
        let collection: Collection<Document> = self.db.collection("userss");
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
                    let mut users: Users = mongodb::bson::from_document(doc).unwrap();
                    users.id = Some(id.to_string());
                    users
                }),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    async fn delete_users(&mut self, id: &str) -> bool {
        let collection: Collection<Document> = self.db.collection("userss");
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

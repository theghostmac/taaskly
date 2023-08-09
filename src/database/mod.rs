use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

use super::config::*;
use super::models::Tasks;

pub struct DatabaseRepository {
    collection: Collection<Tasks>,
}

impl DatabaseRepository {
    pub fn connect() -> Self {
        let conf = Config::from_env();
        let client_options = conf.get_db_client_options();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database("taaskly");
        let col: Collection<Tasks> = db.collection("Tasks");
        Self { collection: col }
    }

    pub fn get_tasks(&self) -> Result<Vec<Tasks>, Error> {
        let tasks = self.collection.find(None, None).ok().unwrap();

        Ok(tasks.map(|i| i.unwrap()).collect())
    }

    pub fn insert_task(&self, task: &Tasks) -> Result<InsertOneResult, Error> {
        let inserted_task = self.collection.insert_one(task, None).ok().unwrap();

        Ok(inserted_task)
    }

    pub fn mark_as_done(&self, id: &String) -> Result<UpdateResult, Error> {
        let oid = ObjectId::parse_str(&id).unwrap();
        let filter = doc! {"_id": oid};
        let new_doc = doc! {
            "$set": {
                "is_done": true,
            }
        };
        let updated_doc = self
            .collection
            .update_one(filter, new_doc, None)
            .ok()
            .unwrap();
        Ok(updated_doc)
    }

    pub fn remove_task(&self, id: &String) -> Result<DeleteResult, Error> {
        let oid = ObjectId::parse_str(&id).unwrap();
        let filter = doc! {"_id": oid};
        let deleted_doc = self.collection.delete_one(filter, None).ok().unwrap();
        Ok(deleted_doc)
    }
}

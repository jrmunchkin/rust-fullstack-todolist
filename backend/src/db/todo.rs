use super::DB;
use crate::error::Error;
use crate::error::Error::*;
use futures::StreamExt;
use model::*;
use mongodb::{
    bson::{doc, document::Document, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
};

const COLLECTION: &str = "todo";
const ID: &str = "_id";
const NAME: &str = "name";
const IS_COMPLETE: &str = "is_complete";

impl DB {
    pub async fn fetch_todos(&self) -> Result<Vec<Todo>, Error> {
        let collection = self.get_database().collection(COLLECTION);

        let mut cursor = collection.find(None, None).await.map_err(MongoQueryError)?;

        let mut todos: Vec<Todo> = Vec::new();

        while let Some(doc) = cursor.next().await {
            todos.push(self.doc_to_todo(&doc?)?);
        }

        Ok(todos)
    }

    pub async fn create_todo(&self, entry: &TodoCreateRequest) -> Result<InsertOneResult, Error> {
        let collection = self.get_database().collection(COLLECTION);

        let doc = doc! {
            NAME: entry.name.clone(),
            IS_COMPLETE: entry.is_complete.clone(),
        };

        let todo = collection
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(todo)
    }

    pub async fn edit_todo(
        &self,
        id: &str,
        entry: &TodoUpdateRequest,
    ) -> Result<UpdateResult, Error> {
        let collection = self.get_database().collection::<Todo>(COLLECTION);
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let query = doc! {
            ID: oid,
        };

        let doc = doc! {
            "$set": {
                IS_COMPLETE: entry.is_complete.clone(),
            }
        };

        let todo = collection
            .update_one(query, doc, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(todo)
    }

    pub async fn delete_todo(&self, id: &str) -> Result<DeleteResult, Error> {
        let collection = self.get_database().collection::<Todo>(COLLECTION);
        let oid = ObjectId::parse_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let query = doc! {
            ID: oid,
        };

        let todo = collection
            .delete_one(query, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(todo)
    }

    pub async fn delete_all_todos(&self) -> Result<DeleteResult, Error> {
        let collection = self.get_database().collection::<Todo>(COLLECTION);

        let todo = collection
            .delete_many(doc! {}, None)
            .await
            .map_err(MongoQueryError)?;

        Ok(todo)
    }

    fn doc_to_todo(&self, doc: &Document) -> Result<Todo, Error> {
        let id = doc.get_object_id(ID)?;
        let name = doc.get_str(NAME)?;
        let is_complete = doc.get_bool(IS_COMPLETE)?;

        Ok(Todo {
            id: id.to_hex(),
            name: name.to_owned(),
            is_complete: is_complete.to_owned(),
        })
    }
}

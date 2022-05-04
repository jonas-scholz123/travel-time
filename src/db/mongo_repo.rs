use futures::{
    stream::{StreamExt, TryStreamExt},
    Stream,
};
use std::collections;

use anyhow::Result;
use mongodb::{
    bson::doc,
    options::{FindOneOptions, InsertOneOptions, ReplaceOptions},
    Client, Collection, Cursor,
};

use serde::{de::DeserializeOwned, Serialize};

use super::mongo_doc::MongoDoc;

pub struct MongoRepository<T> {
    pub collection: Collection<T>,
}

impl<T> MongoRepository<T>
where
    T: Serialize + DeserializeOwned + MongoDoc + Unpin + Send + Sync,
{
    pub fn new(client: &Client) -> Self {
        let collection = client
            .database(T::database_name())
            .collection(T::collection_name());
        Self { collection }
    }

    pub async fn get_by_id<S: Into<String>>(&self, id: S) -> Result<Option<T>> {
        let filter = doc! {"_id": id.into()};
        let options = FindOneOptions::default();
        Ok(self.collection.find_one(filter, options).await?)
    }

    pub async fn insert(&self, doc: &T) -> Result<()> {
        self.collection
            .insert_one(doc, InsertOneOptions::default())
            .await?;
        Ok(())
    }

    pub async fn insert_or_replace(&self, doc: &T) -> Result<()> {
        let filter = doc! {"_id": doc.id()};
        let options = ReplaceOptions::builder().upsert(true).build();
        self.collection.replace_one(filter, doc, options).await?;
        Ok(())
    }

    pub async fn get_all(&self) -> Result<Cursor<T>> {
        let filter = doc! {};
        Ok(self.collection.find(filter, None).await?)
    }
}

use std::env;
extern crate dotenv;
use dotenv::dotenv;
use futures::stream::TryStreamExt;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    options::FindOptions,
    Client, Collection,
};

use crate::models::data_model::Data;

pub struct MongoRepo {
    col: Collection<Data>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let client_uri =
            env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

        let client = Client::with_uri_str(client_uri)
            .await
            .expect("Connection failed...");
        let db = client.database("Test");
        let col: Collection<Data> = db.collection("datas");
        println!("Connection established...");
        MongoRepo { col }
    }

    pub async fn get_data(&self, id: &String) -> Result<Data, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let function_model = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(function_model.unwrap())
    }

    pub async fn get_all_data(&self) -> Result<Vec<Data>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of datas");
        let mut datas: Vec<Data> = Vec::new();
        while let Some(data) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            datas.push(data)
        }
        Ok(datas)
    }
}

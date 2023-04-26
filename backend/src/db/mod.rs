pub mod todo;

use crate::error::Error;
use crate::{DB_NAME, MONGO_URL};
use mongodb::{bson::doc, options::ClientOptions, Client, Database};

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self, Error> {
        let mut client_options = ClientOptions::parse(MONGO_URL).await?;

        client_options.app_name = Some("Todo list".to_string());

        let client = Client::with_options(client_options)?;

        client
            .database(DB_NAME)
            .run_command(doc! {"ping": 1}, None)
            .await?;

        Ok(Self { client })
    }

    fn get_database(&self) -> Database {
        self.client.database(DB_NAME)
    }
}

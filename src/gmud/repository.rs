use crate::gmud::models::GmudModel;
use futures_util::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime as BsonDateTime},
    error::Error,
    Collection,
};

#[derive(Clone)]
pub struct GmudRepository {
    collection: Collection<GmudModel>,
}
impl GmudRepository {
    pub fn new(collection: Collection<GmudModel>) -> Self {
        Self { collection }
    }

    pub async fn find_all(&self) -> Result<Vec<GmudModel>, Error> {
        let mut cursor = self.collection.find(doc! {}).await?;

        let gmuds = cursor.try_collect::<Vec<GmudModel>>().await?;
        Ok(gmuds)
    }

    pub async fn find_by_id(
        &self,
        id: &str,
    ) -> Result<Option<GmudModel>, Box<dyn std::error::Error>> {
        let object_id = ObjectId::parse_str(id)?;

        let filter = doc! { "_id": object_id };
        let gmud = self.collection.find_one(filter).await?;
        Ok(gmud)
    }

    pub async fn create(&self, mut nova_gmud: GmudModel) -> Result<ObjectId, Error> {
        nova_gmud.created_at = chrono::Utc::now().to_rfc3339();
        nova_gmud.updated_at = chrono::Utc::now().to_rfc3339();

        let result = self.collection.insert_one(nova_gmud).await?;

        let inserted_id = result.inserted_id.as_object_id().unwrap();
        Ok(inserted_id)
    }

    pub async fn update_status(
        &self,
        id: &str,
        novo_status: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let object_id = ObjectId::parse_str(id)?;

        let filter = doc! { "_id": object_id };
        let update = doc! {
            "$set": {
                "status": novo_status,
                "updated_at": BsonDateTime::now()
            }
        };

        let result = self.collection.update_one(filter, update).await?;
        Ok(result.modified_count > 0)
    }
}

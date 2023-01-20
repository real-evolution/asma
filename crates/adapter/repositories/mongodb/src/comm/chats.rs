use chrono::Utc;
use futures::{stream::BoxStream, StreamExt};
use kernel_entities::{
    entities::{
        auth::User,
        comm::{Chat, Message},
    },
    traits::Key,
};
use kernel_repositories::{
    comm::{ChatsRepo, InsertChat},
    error::RepoResult,
    traits::InsertRepo,
};
use mongodb::{
    bson::{doc, Document},
    options::{ChangeStreamOptions, FullDocumentType},
};

use crate::{
    repo::MongoDbRepo,
    traits::collection_entity::CollectionEntity,
    util::error::map_mongo_error,
};

#[async_trait::async_trait]
impl ChatsRepo for MongoDbRepo<Chat> {
    async fn watch(
        &self,
        id: &Key<Chat>,
    ) -> RepoResult<BoxStream<'_, RepoResult<Message>>> {
        self.watch_messages(doc! { "fullDocument.chat_id": id.to_string() })
            .await
    }

    async fn watch_all_of(
        &self,
        user_id: &Key<User>,
    ) -> RepoResult<BoxStream<'static, RepoResult<Message>>> {
        self.watch_messages(
            doc! { "fullDocument.user_id": user_id.to_string() },
        )
        .await
    }
}

#[async_trait::async_trait]
impl InsertRepo<InsertChat> for MongoDbRepo<Chat> {
    async fn create(&self, model: InsertChat) -> RepoResult<Self::Entity> {
        let chat = Chat {
            id: uuid::Uuid::new_v4().into(),
            label: model.label,
            state: model.state,
            user_id: model.user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.collection()
            .insert_one(&chat, None)
            .await
            .map_err(map_mongo_error)?;

        Ok(chat)
    }
}

impl MongoDbRepo<Chat> {
    async fn watch_messages<F: Into<Document>>(
        &self,
        filter: F,
    ) -> RepoResult<BoxStream<'static, RepoResult<Message>>> {
        let pipeline = vec![doc! {
            "$match": {
                "$and": [
                    filter.into(),
                    { "operationType": "insert" }
                ]
            }
        }];

        let opts = ChangeStreamOptions::builder()
            .full_document(Some(FullDocumentType::Required))
            .build();

        Ok(self
            .database
            .collection(Message::name())
            .watch(pipeline, opts)
            .await
            .map_err(map_mongo_error)?
            .filter_map(|e| async move {
                match e {
                    | Ok(event) => match event.full_document {
                        | Some(doc) => Some(Ok(doc)),
                        | None => None,
                    },
                    | Err(err) => Some(Err(map_mongo_error(err))),
                }
            })
            .boxed())
    }
}

impl CollectionEntity for Chat {
    fn name() -> &'static str {
        "chats"
    }
}

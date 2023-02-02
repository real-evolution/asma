use derive_more::Constructor;
use driver_web_common::{auth::validator::AuthValidator, state::AppState};
use kernel_entities::{
    entities::auth::{Action, Resource, User},
    traits::Entity,
};
use kernel_repositories::traits::StatsRepo;
use tonic::{Request, Response};

use crate::{
    proto::{
        services::{self, stats_server::Stats, GetStatsResponse},
        ProtoResult,
    },
    util::{
        auth::token::{GrpcAuthToken, RequestExt},
        error::IntoStatusResult,
    },
};

#[derive(Constructor)]
pub(crate) struct GrpcStatsService {
    state: AppState,
}

#[tonic::async_trait]
impl Stats for GrpcStatsService {
    async fn get_stats(
        &self,
        req: Request<()>,
    ) -> ProtoResult<Response<GetStatsResponse>> {
        let auth = req.auth(self.state.config.clone())?;

        auth.can(&[
            (Resource::Account, Action::View),
            (Resource::Session, Action::View),
            (Resource::Channel, Action::View),
            (Resource::Chat, Action::View),
            (Resource::Bot, Action::View),
        ])?;

        let (data, docs) = (&self.state.data, &self.state.docs);

        let accounts = data.auth().accounts().stats_for(&auth).await?;
        let sessions = data.auth().sessions().stats_for(&auth).await?;
        let channels = data.link().channels().stats_for(&auth).await?;
        let bots = data.comm().bots().stats_for(&auth).await?;
        let chats = docs.chats().stats_for(&auth).await?;

        let stats = services::GetStatsResponse {
            accounts,
            sessions,
            channels,
            bots,
            chats,
        };

        Ok(Response::new(stats))
    }
}

#[async_trait::async_trait]
trait StatsHelper {
    async fn stats_for(
        &self,
        auth: &GrpcAuthToken,
    ) -> ProtoResult<Option<services::StatsPair>>;
}

#[async_trait::async_trait]
impl<E, R> StatsHelper for R
where
    E: Entity,
    R: StatsRepo<User, Entity = E> + ?Sized,
{
    async fn stats_for(
        &self,
        auth: &GrpcAuthToken,
    ) -> ProtoResult<Option<services::StatsPair>> {
        let pair = self
            .get_stats_for(&auth.user_id)
            .await
            .into_status_result()?;

        Ok(Some(services::StatsPair {
            total: pair.total,
            active: pair.active,
        }))
    }
}

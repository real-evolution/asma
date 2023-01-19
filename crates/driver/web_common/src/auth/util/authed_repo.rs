use chrono::{DateTime, Utc};
use kernel_entities::{
    entities::auth::Action,
    traits::{Entity, Key},
    util::ResourceEntity,
};
use kernel_repositories::{error::RepoError, traits::Repo};

use crate::auth::validator::AuthValidator;

#[async_trait::async_trait]
pub trait AuthedRepoExts<E: Entity>: Repo<Entity = E> {
    async fn get_authed<V, VE>(
        &self,
        key: &Key<Self::Entity>,
        auth: &V,
    ) -> Result<Self::Entity, VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>;

    async fn get_paginated_authed<V, VE>(
        &self,
        before: &DateTime<Utc>,
        limit: usize,
        auth: &V,
    ) -> Result<Vec<Self::Entity>, VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>;

    async fn exists_authed<V, VE>(
        &self,
        key: &Key<Self::Entity>,
        auth: &V,
    ) -> Result<bool, VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>;

    async fn remove_authed<V, VE>(
        &self,
        key: &Key<Self::Entity>,
        auth: &V,
    ) -> Result<(), VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>;
}

#[async_trait::async_trait]
impl<E, R> AuthedRepoExts<E> for R
where
    R: Repo<Entity = E> + ?Sized,
    E: ResourceEntity,
{
    async fn get_authed<V, VE>(
        &self,
        key: &Key<Self::Entity>,
        auth: &V,
    ) -> Result<Self::Entity, VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>,
    {
        auth.can(&[(E::resource(), Action::View)])?;

        self.get(key).await.map_err(From::from)
    }

    async fn get_paginated_authed<V, VE>(
        &self,
        before: &DateTime<Utc>,
        limit: usize,
        auth: &V,
    ) -> Result<Vec<Self::Entity>, VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>,
    {
        auth.can(&[(E::resource(), Action::View)])?;

        self.get_paginated(before, limit).await.map_err(From::from)
    }

    async fn exists_authed<V, VE>(
        &self,
        key: &Key<Self::Entity>,
        auth: &V,
    ) -> Result<bool, VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>,
    {
        auth.can(&[(E::resource(), Action::View)])?;

        self.exists(key).await.map_err(From::from)
    }

    async fn remove_authed<V, VE>(
        &self,
        key: &Key<Self::Entity>,
        auth: &V,
    ) -> Result<(), VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>,
    {
        auth.can(&[(E::resource(), Action::Remove)])?;

        self.remove(key).await.map_err(From::from)
    }
}

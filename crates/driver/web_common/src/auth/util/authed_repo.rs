use chrono::{DateTime, Utc};
use kernel_entities::{
    entities::auth::{Action, KnownRoles, User},
    traits::{Entity, Key},
    util::ResourceEntity,
};
use kernel_repositories::{
    error::RepoError,
    traits::{ChildRepo, InsertRepo, Repo},
};

use crate::auth::validator::AuthValidator;

#[async_trait::async_trait]
pub trait RepoExt<E: Entity>: Repo<Entity = E> {
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
pub trait InsertRepoExt<'a, E, I>: InsertRepo<I, Entity = E>
where
    I: 'a,
{
    async fn create_authed<V, VE>(
        &'a self,
        model: I,
        auth: &V,
    ) -> Result<Self::Entity, VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>;
}

#[async_trait::async_trait]
pub trait ChildRepoExt<P, E>: ChildRepo<P, Entity = E>
where
    P: Entity,
    E: Entity,
{
    async fn get_paginated_of_authed<const ALLOW_ADMIN: bool, V, VE>(
        &self,
        parent_key: &Key<P>,
        before: &DateTime<Utc>,
        limit: usize,
        auth: &V,
    ) -> Result<Vec<Self::Entity>, VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>;

    async fn get_of_authed<const ALLOW_ADMIN: bool, V, VE>(
        &self,
        parent_key: &Key<P>,
        key: &Key<Self::Entity>,
        auth: &V,
    ) -> Result<Self::Entity, VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>;

    async fn remove_of_authed<const ALLOW_ADMIN: bool, V, VE>(
        &self,
        parent_key: &Key<P>,
        key: &Key<Self::Entity>,
        auth: &V,
    ) -> Result<(), VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>;
}

#[async_trait::async_trait]
impl<E, R> RepoExt<E> for R
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

#[async_trait::async_trait]
impl<'a, E, I, R> InsertRepoExt<'a, E, I> for R
where
    R: InsertRepo<I, Entity = E> + ?Sized,
    E: ResourceEntity,
    I: Send + 'a,
{
    async fn create_authed<V, VE>(
        &'a self,
        model: I,
        auth: &V,
    ) -> Result<Self::Entity, VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>,
    {
        auth.can(&[(E::resource(), Action::Add)])?;

        self.create(model).await.map_err(From::from)
    }
}

#[async_trait::async_trait]
impl<E: ResourceEntity, R> ChildRepoExt<User, E> for R
where
    R: ChildRepo<User, Entity = E>,
{
    async fn get_paginated_of_authed<const ALLOW_ADMIN: bool, V, VE>(
        &self,
        parent_key: &Key<User>,
        before: &DateTime<Utc>,
        limit: usize,
        auth: &V,
    ) -> Result<Vec<Self::Entity>, VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>,
    {
        auth.can(&[(E::resource(), Action::View)])?
            .of(parent_key)
            .or_else(|err| {
                if ALLOW_ADMIN {
                    auth.in_role(KnownRoles::Admin)
                } else {
                    Err(err)
                }
            })?;

        self.get_paginated_of(parent_key, before, limit)
            .await
            .map_err(From::from)
    }

    async fn get_of_authed<const ALLOW_ADMIN: bool, V, VE>(
        &self,
        parent_key: &Key<User>,
        key: &Key<Self::Entity>,
        auth: &V,
    ) -> Result<Self::Entity, VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>,
    {
        auth.can(&[(E::resource(), Action::View)])?
            .of(parent_key)
            .or_else(|err| {
                if ALLOW_ADMIN {
                    auth.in_role(KnownRoles::Admin)
                } else {
                    Err(err)
                }
            })?;

        self.get_of(parent_key, key).await.map_err(From::from)
    }

    async fn remove_of_authed<const ALLOW_ADMIN: bool, V, VE>(
        &self,
        parent_key: &Key<User>,
        key: &Key<Self::Entity>,
        auth: &V,
    ) -> Result<(), VE>
    where
        V: AuthValidator<Error = VE>,
        VE: From<RepoError>,
    {
        auth.can(&[(E::resource(), Action::Remove)])?
            .of(parent_key)
            .or_else(|err| {
                if ALLOW_ADMIN {
                    auth.in_role(KnownRoles::Admin)
                } else {
                    Err(err)
                }
            })?;

        self.remove_of(parent_key, key).await.map_err(From::from)
    }
}

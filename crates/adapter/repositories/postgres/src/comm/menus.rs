use chrono::{DateTime, Utc};
use kernel_entities::{
    entities::comm::{Bot, Menu},
    traits::Key,
};
use kernel_repositories::{
    comm::{InsertMenu, MenusRepo},
    error::RepoResult,
    traits::*,
};
use ormx::{Delete, Table};
use proc_macros::Repo;

use crate::{
    database::SqlxPool,
    sqlx_ok,
    sqlx_vec_ok,
    util::error::map_sqlx_error,
};

#[derive(Repo)]
#[repo(
    table = "menus",
    read(entity = "Menu", model = "models::MenuModel"),
    insert(entity = "InsertMenu", model = "models::InsertMenuModel")
)]
pub(crate) struct SqlxMenusRepo(pub SqlxPool);

#[async_trait::async_trait]
impl MenusRepo for SqlxMenusRepo {
    async fn get_submenus(&self, id: &Key<Menu>) -> RepoResult<Vec<Menu>> {
        sqlx_vec_ok!(
            sqlx::query_as!(
                models::MenuModel,
                r#"
                SELECT * FROM menus
                WHERE parent_menu_id = $1 AND
                      parent_menu_id != id AND
                      is_active = TRUE
                "#,
                id.value_ref(),
            )
            .fetch_all(self.0.get())
            .await
        )
    }

    async fn get_with_submenus(
        &self,
        id: &Key<Menu>,
    ) -> RepoResult<(Menu, Vec<Menu>)> {
        Ok((self.get(id).await?, self.get_submenus(id).await?))
    }
}

#[async_trait::async_trait]
impl ChildRepo<Bot> for SqlxMenusRepo {
    async fn get_paginated_of(
        &self,
        bot_id: &Key<Bot>,
        before: &DateTime<Utc>,
        limit: usize,
    ) -> RepoResult<Vec<Self::Entity>> {
        sqlx_vec_ok!(
            sqlx::query_as!(
                models::MenuModel,
                r#"
                SELECT * FROM menus
                WHERE bot_id = $1 AND created_at <= $2
                ORDER BY created_at
                LIMIT $3
                "#,
                bot_id.value_ref(),
                before,
                limit as i64
            )
            .fetch_all(self.0.get())
            .await
        )
    }

    async fn get_of(
        &self,
        bot_id: &Key<Bot>,
        id: &Key<Menu>,
    ) -> RepoResult<Self::Entity> {
        sqlx_ok!(
            sqlx::query_as!(
                models::MenuModel,
                r#"SELECT * FROM menus WHERE id = $1 AND bot_id = $2"#,
                id.value_ref(),
                bot_id.value_ref()
            )
            .fetch_one(self.0.get())
            .await
        )
    }

    async fn remove_of(
        &self,
        bot_id: &Key<Bot>,
        id: &Key<Self::Entity>,
    ) -> RepoResult<()> {
        sqlx::query_as!(
            models::MenuModel,
            r#"DELETE FROM menus WHERE id = $1 AND bot_id = $2"#,
            id.value_ref(),
            bot_id.value_ref()
        )
        .fetch_one(self.0.get())
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}

mod models {
    use chrono::{DateTime, Utc};
    use derive_more::{From, Into};
    use kernel_entities::{entities::comm::Menu, traits::KeyType};
    use kernel_repositories::comm::InsertMenu;

    use crate::generate_mapping;

    #[derive(Clone, Debug, From, Into, ormx::Table)]
    #[ormx(table = "menus", id = id, insertable, deletable)]
    pub struct MenuModel {
        pub id: KeyType,
        pub title: String,
        pub content: Option<String>,
        pub menu_trigger: String,
        pub matching_strategy: i32,
        #[ormx(set)]
        pub is_active: bool,
        pub parent_menu_id: KeyType,
        #[ormx(get_many)]
        pub bot_id: KeyType,
        #[ormx(default)]
        pub created_at: DateTime<Utc>,
        #[ormx(default, set)]
        pub updated_at: DateTime<Utc>,
    }

    impl From<InsertMenu> for InsertMenuModel {
        fn from(val: InsertMenu) -> Self {
            let id = uuid::Uuid::new_v4();

            Self {
                id,
                title: val.title,
                content: val.content,
                menu_trigger: val.menu_trigger,
                matching_strategy: val.matching_strategy.repr(),
                is_active: val.is_active,
                parent_menu_id: val
                    .parent_menu_id
                    .map(|v| v.value())
                    .unwrap_or(id),
                bot_id: val.bot_id.value(),
            }
        }
    }

    generate_mapping!(Menu, MenuModel, 10);
}

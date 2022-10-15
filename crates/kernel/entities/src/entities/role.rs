use crate::traits::*;
use kernel_proc_macros::*;

#[entity]
#[derive(sqlx::FromRow)]
pub struct Role {
    pub code: String,
    pub friendly_name: Option<String>,
    pub enabled: bool,
}

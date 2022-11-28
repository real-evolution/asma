use chrono::{DateTime, Utc};
use common_macros::into_fn;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

into_fn!(default_before_timestamp: DateTime<Utc> =>  Utc::now());
into_fn!(default_page_size: usize =>  32usize);

#[derive(Deserialize, IntoParams, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    #[serde(default = "default_before_timestamp")]
    pub before: DateTime<Utc>,

    #[serde(default = "default_page_size")]
    #[validate(range(min = 1, max = 128))]
    pub page_size: usize,
}

impl Into<(DateTime<Utc>, usize)> for Pagination {
    fn into(self) -> (DateTime<Utc>, usize) {
        (self.before, self.page_size)
    }
}

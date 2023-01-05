use aide::OperationIo;
use chrono::{DateTime, Utc};
use common_macros::into_fn;
use schemars::JsonSchema;
use serde::Deserialize;
use validator::Validate;

into_fn!(default_before_timestamp: DateTime<Utc> =>  Utc::now());
into_fn!(default_page_size: usize =>  32usize);

#[derive(Deserialize, Validate, JsonSchema, OperationIo)]
#[serde(rename_all = "camelCase")]
#[aide(input)]
pub struct Pagination {
    #[serde(default = "default_before_timestamp")]
    pub before: DateTime<Utc>,

    #[serde(default = "default_page_size")]
    #[validate(range(min = 1, max = 128))]
    pub page_size: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            before: default_before_timestamp(),
            page_size: default_page_size(),
        }
    }
}

impl From<Pagination> for (DateTime<Utc>, usize) {
    fn from(val: Pagination) -> Self {
        (val.before, val.page_size)
    }
}

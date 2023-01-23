use std::str::FromStr;

use driver_web_common::value_types::Pagination;
use kernel_entities::traits::Key;
use tonic::Status;

use crate::proto::value_types::TimePagination;

pub(crate) trait TryConvertInto<T> {
    fn try_convert(self) -> Result<T, Status>;
}

pub(crate) trait TryConvertIntoOrDefault<T> {
    fn try_convert_or_default(self) -> T;
}

impl<E> TryConvertInto<Key<E>> for String {
    fn try_convert(self) -> Result<Key<E>, Status> {
        match Key::from_str(&self) {
            | Ok(key) => Ok(key),
            | Err(err) => {
                warn!("invalid key value `{self}`: {err:#?}");
                Err(Status::invalid_argument("invalid key value"))
            }
        }
    }
}

impl TryConvertInto<Pagination> for Option<TimePagination> {
    fn try_convert(self) -> Result<Pagination, Status> {
        let Some(value) = self else {
            return Ok(Pagination::default());
        };

        Ok(Pagination {
            before: value.before.unwrap_or_default().into(),
            page_size: value.page_size as usize,
        })
    }
}

impl<T, U> TryConvertInto<U> for Option<T>
where
    T: TryConvertInto<U>,
{
    fn try_convert(self) -> Result<U, Status> {
        match self {
            | Some(value) => value.try_convert(),
            | None => {
                Err(Status::failed_precondition("missing required value"))
            }
        }
    }
}

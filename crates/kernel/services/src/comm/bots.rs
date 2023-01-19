use futures::stream::BoxStream;

use crate::error::AppResult;

pub trait BotsService {
    fn start_all(&self) -> BoxStream<'_, AppResult<()>>;
    fn stop_all(&self) -> BoxStream<'_, AppResult<()>>;
}

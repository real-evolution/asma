mod bots;

use aide::axum::ApiRouter;
use driver_web_common::state::AppState;

pub fn routes() -> ApiRouter<AppState> {
    ApiRouter::new().nest("/bots", bots::routes())
}

use aide::OperationIo;
use derive_more::{From, Into};
use driver_web_common::auth::config::AuthTokenConfig;

#[derive(Debug, Clone, From, Into, OperationIo)]
#[aide(input)]
#[repr(transparent)]
pub struct RestAuthTokenConfig(AuthTokenConfig);

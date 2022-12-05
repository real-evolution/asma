use derive_more::Constructor;
use kernel_entities::entities::link::Channel;
use serde::{Deserialize, Serialize};

#[derive(Constructor, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelDto {
    #[serde(flatten)]
    pub channel: Channel,
}

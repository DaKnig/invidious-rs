use crate::{structs::hidden::ChannelVideo, traits::PublicItems};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelVideos {
    pub videos: Vec<ChannelVideo>,
}

impl PublicItems for ChannelVideos {
    fn url(server: &str, args: String) -> String {
        format!("{server}/api/v1/channels/videos/{args}")
    }

    // fn from_value(value: Value) -> Result<Self, serde_json::Error>
    // where
    //     Self: Sized + DeserializeOwned,
    // {
    //     Ok(serde_json::from_value(value)?)
    // }
}

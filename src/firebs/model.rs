use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::openai::model::OpenAIResult;
use std::time::{ SystemTime };

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChannelSet {
    pub channel1: Option<String>,
    pub channel2: Option<String>,
    pub channel3: Option<String>,
    pub channel4: Option<String>,
}

impl ChannelSet {
    pub fn new(channel1: Option<String>,
               channel2: Option<String>,
               channel3: Option<String>,
               channel4: Option<String>
    ) -> ChannelSet {
        ChannelSet { channel1, channel2, channel3, channel4 }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataSet {
    pub id: String,
    pub account: String,
    pub channels: ChannelSet,
    pub prompt: String,
    pub completion: OpenAIResult,
    pub image: String,
    pub ts_time: SystemTime
}

impl DataSet {
    pub fn new(account: String,
               prompt: String,
               completion: OpenAIResult,
               image: String,
               channels: ChannelSet
    ) -> DataSet {
        let id = Uuid::new_v4().to_string();
        let ts_time = SystemTime::now();
        DataSet { id, account, channels,
                  prompt, completion, image,
                  ts_time }
    }
}
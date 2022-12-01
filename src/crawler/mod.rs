use thirtyfour::prelude::*;
use crate::utils::error::Result;
use thirtyfour::WebDriver;
use serde::{Deserialize, Serialize};


pub mod ytb_crawler;
pub mod ytb_crawler_with_swas;
pub mod ytb_download;

pub async fn create_thirtyfour()->Result<WebDriver>{
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    Ok(driver)
}


#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
struct YtbPlayerInfo {
  #[serde(rename = "streamingData")]
  streaming_data: Option<StreamingDataInfo>,
  #[serde(rename = "videoDetails")]
  video_details: Option<VideoDetailsInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
struct StreamingDataInfo {
  #[serde(rename = "adaptiveFormats")]
  adaptive_formats:Option<Vec<FormatsDataInfo>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
struct FormatsDataInfo {
  itag: Option<i32>,
  url: Option<String>,
  #[serde(rename = "mimeType")]
  mime_type:Option<String>,
  width:Option<i32>,
  height:Option<i32>,
  #[serde(rename = "contentLength")]
  content_length:Option<String>,
  quality:Option<String>,
  fps:Option<i32>,
  #[serde(rename = "qualityLabel")]
  quality_label:Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
struct VideoDetailsInfo {
  #[serde(rename = "videoId")]
  video_id:Option<String>,
  title:Option<String>,
  #[serde(rename = "channelId")]
  channel_id:Option<String>,
  #[serde(rename = "shortDescription")]
  short_description:Option<String>,
  author:Option<String>,
  #[serde(rename = "viewCount")]
  view_count:Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
struct ReqClient {
  #[serde(rename = "clientName")]
  client_name: Option<String>,
  #[serde(rename = "clientVersion")]
  client_version: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
struct ReqClientObj {
  client: Option<ReqClient>,
}
#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
struct ReqBody {
    context: Option<ReqClientObj>,
    #[serde(rename = "videoId")]
    video_id: Option<String>,
}
use serde::{Deserialize, Serialize};
use rbatis::DateTimeNative;
use crate::model::entitys::ytb_entity::{YtbInfo,YtbDownload};

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct YtbDTO {
    pub id: Option<i64>,
    pub ytb_id: Option<String>,
    pub ytb_link: Option<String>,
    pub ytb_img_link: Option<String>,
    pub ytb_img_height:Option<i32>,
    pub ytb_img_width:Option<i32>,
    pub ytb_channel:Option<String>,
    pub ytb_duration : Option<String>,
    pub ytb_tips: Option<String>,
    pub ytb_name: Option<String>,
    pub ytb_country: Option<String>,
    pub ytb_author: Option<String>,
    pub ytb_middle: Option<String>,
    pub ytb_high: Option<String>,
    pub img_file_name: Option<String>,
    pub img_file_path: Option<String>,
    pub times: Option<i8>,
    pub status: Option<i8>,
    pub is_public: Option<i8>,
    pub created_at:Option<DateTimeNative>,
    pub updated_at:Option<DateTimeNative>,
}

impl Into<YtbInfo> for YtbDTO {
    fn into(self) -> YtbInfo {
        YtbInfo {
            id: self.id().clone(),
            ytb_id: self.ytb_id().clone(),
            ytb_link: self.ytb_link().clone(),
            ytb_img_link: self.ytb_img_link().clone(),
            ytb_img_height: self.ytb_img_height().clone(),
            ytb_img_width: self.ytb_img_width().clone(),
            ytb_channel: self.ytb_channel().clone(),
            ytb_duration: self.ytb_duration().clone(),
            ytb_tips: self.ytb_tips().clone(),
            ytb_name: self.ytb_name().clone(),
            ytb_country: self.ytb_country().clone(),
            ytb_author: self.ytb_author().clone(),
            ytb_middle: self.ytb_middle().clone(),
            ytb_high: self.ytb_high().clone(),
            img_file_name: self.img_file_name().clone(),
            img_file_path: self.img_file_path().clone(),
            status : self.status().clone(),
            times: self.times().clone(),
            is_public: self.is_public().clone(),
            created_at: self.created_at().clone(),
            updated_at:self.updated_at().clone(),
        }
    }
}

impl From<YtbInfo> for YtbDTO {
    fn from(arg: YtbInfo) -> Self {
        Self {
            id: arg.id,
            ytb_id: arg.ytb_id,
            ytb_link: arg.ytb_link,
            ytb_img_link: arg.ytb_img_link,
            ytb_img_height:arg.ytb_img_height,
            ytb_img_width:arg.ytb_img_width,
            ytb_channel:arg.ytb_channel,
            ytb_duration: arg.ytb_duration,
            ytb_tips: arg.ytb_tips,
            ytb_name: arg.ytb_name,
            ytb_country: arg.ytb_country,
            ytb_author: arg.ytb_author,
            ytb_middle: arg.ytb_middle,
            ytb_high: arg.ytb_high,
            img_file_name: arg.img_file_name,
            img_file_path: arg.img_file_path,
            times: arg.times,
            status: arg.status,
            is_public: arg.is_public,
            created_at: arg.created_at,
            updated_at: arg.updated_at,
        }
    }
}



#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct YtbDownloadDTO {
    pub id: Option<i64>,
    pub ytb_id: Option<String>,
    pub ytb_middle_url: Option<String>,
    pub ytb_mid_width: Option<i32>,
    pub ytb_mid_height: Option<i32>,
    pub ytb_high_url: Option<String>,
    pub ytb_high_width: Option<i32>,
    pub ytb_high_height: Option<i32>,
    pub file_name: Option<String>,
    pub file_path: Option<String>,
    pub hd_file_name: Option<String>,
    pub hd_file_path: Option<String>,
    pub is_download: Option<i8>,
    pub created_at:Option<DateTimeNative>,
}

impl Into<YtbDownload> for YtbDownloadDTO {
    fn into(self) -> YtbDownload {
        YtbDownload {
            id: self.id().clone(),
            ytb_id: self.ytb_id().clone(),
            ytb_middle_url: self.ytb_middle_url().clone(),
            ytb_mid_width: self.ytb_mid_width().clone(),
            ytb_mid_height: self.ytb_mid_height().clone(),
            ytb_high_url: self.ytb_high_url().clone(),
            ytb_high_width: self.ytb_high_width().clone(),
            ytb_high_height: self.ytb_high_height().clone(),
            file_name: self.file_name().clone(),
            file_path: self.file_path().clone(),
            hd_file_name: self.hd_file_name().clone(),
            hd_file_path: self.hd_file_path().clone(),
            is_download: self.is_download().clone(),
            created_at:self.created_at().clone(),
        }
    }
}

impl From<YtbDownload> for YtbDownloadDTO {
    fn from(arg: YtbDownload) -> Self {
        Self {
            id: arg.id,
            ytb_id: arg.ytb_id,
            ytb_middle_url: arg.ytb_middle_url,
            ytb_mid_width:arg.ytb_mid_width,
            ytb_mid_height:arg.ytb_mid_height,
            ytb_high_url: arg.ytb_high_url,
            ytb_high_width: arg.ytb_high_width,
            ytb_high_height: arg.ytb_high_height,
            file_name: arg.file_name,
            file_path: arg.file_path,
            hd_file_name: arg.hd_file_name,
            hd_file_path: arg.hd_file_path,
            is_download: arg.is_download,
            created_at:arg.created_at,
        }
    }
}

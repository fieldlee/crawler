use rbatis::DateTimeNative;

#[crud_table(table_name:ytb_info)]
#[derive(Clone, Debug,Default)]
pub struct YtbInfo {
    pub id: Option<i64>,
    pub ytb_id: Option<String>,
    pub ytb_link: Option<String>,
    pub ytb_img_link: Option<String>,
    pub ytb_img_height: Option<i32>,
    pub ytb_img_width: Option<i32>,
    pub ytb_channel: Option<String>,
    pub ytb_duration: Option<String>,
    pub ytb_tips: Option<String>,
    pub ytb_name: Option<String>,
    pub ytb_country: Option<String>,
    pub ytb_author: Option<String>,
    pub times: Option<i8>,
    pub status: Option<i8>,
    pub is_public: Option<i8>,
    pub created_at:Option<DateTimeNative>,
    pub updated_at:Option<DateTimeNative>,
}

impl_field_name_method!(YtbInfo {
    id,
    ytb_id,
    ytb_link,
    ytb_img_link,
    ytb_img_height,
    ytb_img_width,
    ytb_channel,
    ytb_duration,
    ytb_tips,
    ytb_name,
    ytb_country,
    ytb_author,
    times,
    status
    is_public,
    created_at,
    updated_at,
});


#[crud_table(table_name:ytb_dl)]
#[derive(Clone, Debug,Default)]
pub struct YtbDownload { 
    pub id: Option<i64>,
    pub ytb_id: Option<String>,
    pub ytb_middle_url: Option<String>,
    pub ytb_high_url: Option<String>,
    pub file_name: Option<String>,
    pub file_path: Option<String>,
    pub is_download: Option<i8>,
    pub created_at:Option<DateTimeNative>,
}

impl_field_name_method!(YtbDownload {
    id,
    ytb_id,
    ytb_middle_url,
    ytb_high_url,
    file_name,
    file_path,
    is_download,
    created_at,
});
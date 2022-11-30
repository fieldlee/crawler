use serde::{Deserialize, Serialize};
use rbatis::DateTimeNative;
use crate::model::entitys::crawler_entity::CrawlerInfo;

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct CrawlerDTO {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub link: Option<String>,
    pub search_word: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub created_at:Option<DateTimeNative>,
}

impl Into<CrawlerInfo> for CrawlerDTO {
    fn into(self) -> CrawlerInfo {
        CrawlerInfo {
            id: self.id().clone(),
            name: self.name().clone(),
            link: self.link().clone(),
            search_word: self.search_word().clone(),
            username: self.username().clone(),
            password: self.password().clone(),
            created_at: self.created_at().clone(),
        }
    }
}

impl From<CrawlerInfo> for CrawlerDTO {
    fn from(arg: CrawlerInfo) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            link: arg.link,
            search_word: arg.search_word,
            username: arg.username,
            password: arg.password,
            created_at: arg.created_at,
        }
    }
}

use rbatis::DateTimeNative;

#[crud_table(table_name:crawler_info)]
#[derive(Clone, Debug)]
pub struct CrawlerInfo {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub link: Option<String>,
    pub search_word: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub created_at:Option<DateTimeNative>,
}

impl_field_name_method!(CrawlerInfo {
    id,
    name,
    link,
    search_word,
    username,
    password,
    created_at,
});
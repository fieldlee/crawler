use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct YtbQuery {
    ytb_id: Option<String>,
    ytb_links: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct YtbNoResultQuery {
   pub  times: Option<i8>,
   pub status: Option<i8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct CrawlerQuery {
    
}

#[derive(Serialize, Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct YtbDlQuery {
    
}
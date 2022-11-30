
use crate::utils::error::Result;
use crate::model::dto::crawler_dto::CrawlerDTO;
use crate::model::entitys::crawler_entity::CrawlerInfo;
use crate::model::request::CrawlerQuery;
use rbatis::rbatis::Rbatis;
use crate::{crud::CrudService, APPLICATION_CONTEXT};

pub struct CrawlerService;

impl Default for CrawlerService {
    fn default() -> Self {
        CrawlerService {}
    }
}

impl CrawlerService {
    pub async fn get_crawler_list(&self) -> Result<Vec<CrawlerDTO>> {
        let results =  self.list_all().await?;
        return Ok(results);
    }

    pub async fn save_info(&mut self,arg: CrawlerDTO) -> Result<i64> {
        let mut entity:CrawlerInfo = arg.clone().into();
        /*保存到数据库*/
        if let Some(id) = entity.id {
            let user = self.get(id.to_string()).await;
            match user {
                Ok(_) => {
                    self.update_by_id(id.to_string(), &entity).await;
                    return Ok(id);
                },
                Err(err) => {
                    return Ok(0);
                }
            }
        } else {
            let id = self.save(&mut entity).await?;
            return Ok(id);
        }
    }
    
}

impl CrudService<CrawlerInfo,CrawlerDTO,CrawlerQuery> for CrawlerService {
    fn get_wrapper(arg: &CrawlerQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(
        &self,
        common: crate::model::entitys::CommonField,
        data: &mut CrawlerInfo,
    ) {

    }
}



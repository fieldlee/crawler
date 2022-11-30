
use crate::utils::error::Result;
use crate::model::dto::ytb_dto::YtbDTO;
use crate::model::entitys::ytb_entity::YtbInfo;
use crate::model::request::{YtbNoResultQuery};
use rbatis::rbatis::Rbatis;
use crate::{crud::CrudService, APPLICATION_CONTEXT};

pub struct YTBService;

impl Default for YTBService {
    fn default() -> Self {
        YTBService {}
    }
}

impl YTBService {
    pub async fn get_ytb_list(&self) -> Result<Vec<YtbDTO>> {
        let arg = YtbNoResultQuery{
            times:Some(5),
            status:Some(0),
        };
        let results =  self.list(&arg).await?;
        return Ok(results);
    }

    pub async fn get_ytb_by_id(&self,ytb_id:&str) -> Result<YtbDTO> {
        let results =  self.get_by_ytb_id(ytb_id.to_owned()).await?;
        return Ok(results);
    }

    pub async fn save_info(&self,arg: YtbDTO) -> Result<i64> {
        let mut entity:YtbInfo = arg.clone().into();
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

impl CrudService<YtbInfo,YtbDTO,YtbNoResultQuery> for YTBService {
    fn get_wrapper(arg: &YtbNoResultQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper().ge("times", arg.times).eq("status", arg.status())
    }
    fn set_save_common_fields(
        &self,
        common: crate::model::entitys::CommonField,
        data: &mut YtbInfo,
    ) {

    }

}



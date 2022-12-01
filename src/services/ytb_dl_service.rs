
use crate::utils::error::Result;
use crate::model::dto::ytb_dto::YtbDownloadDTO;
use crate::model::entitys::ytb_entity::YtbDownload;
use crate::model::request::YtbDlQuery;
use rbatis::rbatis::Rbatis;
use crate::{crud::CrudService, APPLICATION_CONTEXT};

pub struct YTBDLService;

impl Default for YTBDLService {
    fn default() -> Self {
        YTBDLService {}
    }
}

impl YTBDLService {
    pub async fn get_ytb_list(&self) -> Result<Vec<YtbDownloadDTO>> {
        let arg = YtbDlQuery{
        };
        let results =  self.list(&arg).await?;
        return Ok(results);
    }

    pub async fn get_ytb_by_id(&self,ytb_id:&str) -> Result<YtbDownloadDTO> {
        let results =  self.get_by_ytb_id(ytb_id.to_owned()).await?;
        return Ok(results);
    }

    pub async fn save_info(&self,arg: YtbDownloadDTO) -> Result<i64> {
        let mut entity:YtbDownload = arg.clone().into();
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

impl CrudService<YtbDownload,YtbDownloadDTO,YtbDlQuery> for YTBDLService {
    fn get_wrapper(arg: &YtbDlQuery) -> rbatis::wrapper::Wrapper {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.new_wrapper()
    }
    fn set_save_common_fields(
        &self,
        common: crate::model::entitys::CommonField,
        data: &mut YtbDownload,
    ) {

    }

}



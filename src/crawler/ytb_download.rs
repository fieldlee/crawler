use crate::crud::CrudService;
use crate::utils::error::Result;
use crate::services::ytb_service::YTBService;
use crate::APPLICATION_CONTEXT;
use rustube::{Id, VideoFetcher};

pub async fn download_ytb_async()->Result<()>{
    let ytb_service = APPLICATION_CONTEXT.get::<YTBService>();
    loop {
        //获得未爬成功的视频
        let ytb_list = ytb_service.get_by_ytb_no_status_list_5(1).await?;

        for mut ytb_info in ytb_list{
            // 同步执行下载
            tokio::spawn(async move{
                let ytb_link = ytb_info.ytb_link();
                let id = Id::from_raw(ytb_link.as_ref().unwrap().as_str()).unwrap();

                let descrambler = VideoFetcher::from_id(id.into_owned())
                    .unwrap()
                    .fetch()
                    .await
                    .unwrap();
            
                let view_count = descrambler.video_details().view_count;
                let title = descrambler.video_title();
                let author = descrambler.video_details().clone().author;
                

                println!("The video `{}` was viewed {} times.", title, view_count);

                let buf = rustube::download_worst_quality(ytb_info.ytb_link().clone().unwrap().as_str()).await;

                println!("rustube::downloa:{:?}", buf);

                ytb_info.set_ytb_author(Some(author));
                ytb_info.set_status(Some(1));
                ytb_info.set_times(Some(ytb_info.times().unwrap()+1));

                ytb_service.save_info(ytb_info).await;
            });

        }
    }
    Ok(())
}


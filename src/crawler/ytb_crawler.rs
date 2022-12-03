
use crate::utils::error::Result;
use crate::utils::u64_to_i32;
use crate::services::crawler_service::CrawlerService;
use crate::services::ytb_service::YTBService;
use crate::APPLICATION_CONTEXT;
use crate::model::dto::ytb_dto::YtbDTO;
use thirtyfour::prelude::*;
use crate::utils::random_seconds;
use log::info;
use rbatis::DateTimeNative;
use crate::crawler::ytb_crawler_with_swas::get_data;
use yt_api::ApiKey;
use yt_api::search::SearchList;
use yt_api::search::ItemType;
use yt_api::search::Response;

pub async fn ytb_crawler_swas() ->Result<()>{
    let crawler_service = APPLICATION_CONTEXT.get::<CrawlerService>();   
    let ytb_service = APPLICATION_CONTEXT.get::<YTBService>();
    let crawler_list = crawler_service.get_crawler_list().await?;
    for i in 0..crawler_list.len(){
        let crawler_info = crawler_list.get(i);
             match crawler_info{
                 Some(info) => {
                     println!("{:?}",info);
                     println!("{:?}",info.link().as_ref().unwrap());
                     for i in 1..=100{
                        let result_json = get_data(info.search_word().as_ref().unwrap(),i).await?;
                        println!("{:?}",result_json);

                        let secs =  u64::try_from(random_seconds());
                        std::thread::sleep(std::time::Duration::from_secs(secs.unwrap()));
                     }
                 }, 
                 None => println!("")
             }
     }
     Ok(())
}


pub async fn ytb_crawler()->Result<()>{
    let driver = super::create_thirtyfour().await?;

    let crawler_service = APPLICATION_CONTEXT.get::<CrawlerService>();
    
    let ytb_service = APPLICATION_CONTEXT.get::<YTBService>();

    let crawler_list = crawler_service.get_crawler_list().await?;

    for i in 0..crawler_list.len(){
       let crawler_info = crawler_list.get(i);
            match crawler_info{
                Some(info) => {
                    println!("{:?}",info);
                    println!("{:?}",info.link().as_ref().unwrap());
                    let search_url = format!("{}/results?search_query={}",info.link().as_ref().unwrap(),info.search_word().as_ref().unwrap());
                    driver.goto(search_url).await?;
                    

                    let list_items = driver.find_all(By::Tag("ytd-video-renderer")).await?;

                    for item in list_items {
                        let mut ytb_crawler = YtbDTO::default();
                        let img = item.find(By::XPath("//*[@id='thumbnail']/yt-image/img")).await?;
                        let ytd_img_link = img.attr("src").await?;
                        println!("ytd_img_link{:?}",ytd_img_link);
                        ytb_crawler.ytb_img_link = ytd_img_link;
                        // 视频时长
                        // let duration_item = item.find(By::XPath("//*[@id='overlays']")).await?;
                        // let duraiton_text = duration_item.find(By::XPath("/ytd-thumbnail-overlay-time-status-renderer/span")).await?;
                        // let duraiton_value = duraiton_text.inner_html().await?;
                        // println!("duraiton_value{:?}",duraiton_value.clone());
                        // ytb_crawler.ytb_duration = Some(duraiton_value);

                        // 获得视频的名称
                        let title = item.find(By::XPath("//*[@id='video-title']")).await?;
                        let title_text = title.attr("title").await?;
                        ytb_crawler.ytb_name = title_text;
                        let href = title.attr("href").await?;
                        println!("href：{:?}",href);
                        match href {
                            Some(url) => {
                                if url.starts_with("/"){
                                    // 获得ID和链接 
                                    let parks = url.split("?v=");
                                    let id = parks.last();
                                    ytb_crawler.ytb_id = Some(id.unwrap().to_owned());
                                    ytb_crawler.ytb_link = Some(url.to_owned());
                                }
                            },  
                            None => continue,
                        }

                        // 获得视频博主

                        let channel_name = item.find(By::XPath("//*[@id='text-container']")).await?;
                        let channel_value = channel_name.find(By::XPath("//*[@id='text']/a")).await?;
                        println!("channel_value{:?}",channel_value);
                        let name = channel_value.inner_html().await?;
                        println!("name{:?}",name);
                        ytb_crawler.ytb_author = Some(name.clone());
                        ytb_crawler.created_at = Some(DateTimeNative::now());
                        ytb_crawler.updated_at = Some(DateTimeNative::now());
                        let result  = ytb_service.save_info(ytb_crawler).await?;
                        info!("{}",format!("name:{},result:{}", name, result));
                    }

                    // let secs_bytes = random_seconds().to_be_bytes();
                    let secs =  u64::try_from(random_seconds());
                    std::thread::sleep(std::time::Duration::from_secs(secs.unwrap()));
                },
                None => continue,
            }
    }
    driver.close_window().await?;
    Ok(())
 }

 ///*********************
 /// 
 /// 
 /// 
 ///  */
 pub async fn ytb_crawler_by_api()->Result<i32>{
    let mut crawler_num = 0;
    let crawler_service = APPLICATION_CONTEXT.get::<CrawlerService>();
    let crawler_list = crawler_service.get_crawler_list().await?;
    for i in 0..crawler_list.len(){
        crawler_num += 1;
       let crawler_info = crawler_list.get(i);
            match crawler_info{
                Some(info) => {
                   let save_num = ytb_crawler_youtube3(info.search_word().clone().unwrap().as_str()).await?;
                   println!("ytb_crawler_by_api:{}", save_num);
                },
                None => continue,
            }
    }
    Ok(crawler_num)
 }


 ///*********************
 /// ytb_crawler_youtube3
 /// 通过googleapi search youtube
 ///  */

 pub async fn ytb_crawler_youtube3(key_word: &str) -> Result<i32>{
    println!("ytb_crawler_youtube3  key_word:{} ",key_word);
    let key = APPLICATION_CONTEXT.get::<ApiKey>();
    let mut crawler_num = 0;
    println!("SearchList Key:{:?} ",key);
    let mut result = SearchList::new(key.clone())
        .q(key_word)
        .item_type(ItemType::Video)
        .await?;
    let save_num = save_ytb_info(result.clone()).await?;
    println!("ytb_crawler_youtube3 第几次:{} key_word:{} 保存数量：{}",crawler_num,key_word,save_num);
    loop {
        match result.next_page_token {
            Some(token) => {
                crawler_num += 1;

                result = SearchList::new(key.clone())
                .q(key_word)
                .item_type(ItemType::Video)
                .page_token(token)
                .await?;

                let save_num = save_ytb_info(result.clone()).await?;

                println!("ytb_crawler_youtube3 第几次:{} key_word:{} 保存数量：{}",crawler_num,key_word,save_num);
            },
            None => break,
        }
    }
    Ok(crawler_num)
 }
///************************************************************************************************
/// 
/// save_ytb_info
/// 保存爬取的视频地址 id
/// 
///  */
 async fn  save_ytb_info(result:Response) -> Result<i32>{
    let ytb_service = APPLICATION_CONTEXT.get::<YTBService>();
    let mut save_num = 0;
    for item in  result.items{
        let mut ytb_dto = YtbDTO::default();
        ytb_dto.set_ytb_id(item.id.clone().video_id);

        let ytb_result = ytb_service.get_ytb_by_id(item.id.clone().video_id.unwrap().as_str()).await;
        match ytb_result {
            Ok(_) => continue,
            Err(e) => {
                // 查询不到 不处理
            },
        }
        ytb_dto.set_ytb_link(Some(format!("https://youtube.com/watch?v={}",item.id.video_id.unwrap())));
        ytb_dto.set_ytb_img_link(Some(item.snippet.thumbnails.clone().unwrap().default.unwrap().url));
        ytb_dto.set_ytb_img_height(Some(u64_to_i32(item.snippet.thumbnails.clone().unwrap().default.unwrap().height.unwrap())));
        ytb_dto.set_ytb_img_width(Some(u64_to_i32(item.snippet.thumbnails.clone().unwrap().default.unwrap().width.unwrap())));
        ytb_dto.set_ytb_channel(item.snippet.channel_id);
        ytb_dto.set_ytb_name(item.snippet.title);
        ytb_dto.set_ytb_tips(item.snippet.description);
        ytb_dto.set_times(Some(0));
        ytb_dto.set_status(Some(0));
        ytb_dto.set_created_at(Some(DateTimeNative::now()));
        ytb_dto.set_updated_at(Some(DateTimeNative::now()));
        let result = ytb_service.save_info(ytb_dto).await?;
        save_num+=1;
    }
    Ok(save_num)
}



 #[cfg(test)]
mod ytb_crawler_test {
    use super::*;


    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on(async{$e})
        };
      }

    #[test]
    fn it_works() {
        aw!( ytb_crawler().await);
    }


}
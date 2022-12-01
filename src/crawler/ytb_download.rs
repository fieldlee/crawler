use crate::crud::CrudService;
use crate::utils::error::Result;
use rbatis::DateTimeNative;
use crate::services::ytb_service::YTBService;
use crate::services::ytb_dl_service::YTBDLService;
use crate::APPLICATION_CONTEXT;
use rustube::{Id, VideoFetcher};
use crate::config::config::ApplicationConfig;
use reqwest::header::HeaderMap;
use crate::model::dto::ytb_dto::YtbDownloadDTO;
use crate::crawler::{YtbPlayerInfo,ReqBody};
use serde_json::json;

static YOUTUBE_DOWNLOAD_URL: &'static str = "https://www.youtube.com/youtubei/v1/player";

pub async fn download_ytb_info_async()->Result<()>{
    let ytb_service = APPLICATION_CONTEXT.get::<YTBService>();
    loop {
        //获得未爬成功的视频
        let ytb_list = ytb_service.get_by_ytb_no_status_list_5(1).await?;

        for mut ytb_info in ytb_list{
            // 同步执行下载
            tokio::spawn(async move{
                // let ytb_link = ytb_info.ytb_link();
                

                // ytb_info.set_ytb_author(Some(author));
                // ytb_info.set_status(Some(1));
                // ytb_info.set_times(Some(ytb_info.times().unwrap()+1));

                // ytb_service.save_info(ytb_info).await;
            });

        }
    }
    Ok(())
}


pub async fn get_ytb_info(vid : &str){
    let ytb_dl_service = APPLICATION_CONTEXT.get::<YTBDLService>();

    let client = reqwest::Client::new();

    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Goog-Api-Key", config.api_key().parse().unwrap());
    // 组装json
    // 组装要提交的数据
    
    let data = json!({"context":{"client":{"clientName":"ANDROID","clientVersion":"16.05"}},"videoId":""});

    let mut req_body: ReqBody = serde_json::from_value(data).unwrap();

    req_body.video_id = Some(vid.to_string());

    let ytb_dw_info =client.post(format!("{}",YOUTUBE_DOWNLOAD_URL)).headers(headers).json(&req_body).send().await;

    println!("{:?}",ytb_dw_info);
    match ytb_dw_info {
        Ok(resp)=>{
             
            let ytb_play_info: YtbPlayerInfo = serde_json::from_value(resp.json().await.unwrap()).unwrap();

            let mut hd_url = "".to_string();
            let mut mid_url = "".to_string();
            for item in ytb_play_info.streaming_data {
                let formats = item.adaptive_formats.unwrap();
                for adap_item in  formats {
                    if adap_item.quality.is_some() {
                        if adap_item.quality.clone().unwrap() == "medium".to_string()  
                        && adap_item.mime_type.clone().unwrap().contains("mp4")
                        {
                            mid_url = adap_item.url.clone().unwrap();
                        }
                        if adap_item.quality.unwrap().starts_with("hd") 
                        && adap_item.mime_type.unwrap().contains("mp4")
                        {
                            hd_url = adap_item.url.unwrap();
                        }
                    }
                }
            }

            let  mut ytb_dl_info =  YtbDownloadDTO::default();
            
            ytb_dl_info.set_ytb_high_url(Some(hd_url));
            ytb_dl_info.set_ytb_middle_url(Some(mid_url));
            ytb_dl_info.set_ytb_id(Some(vid.to_string()));
            ytb_dl_info.set_is_download(Some(0));
            ytb_dl_info.set_created_at(Some(DateTimeNative::now()));
            ytb_dl_info.set_file_name(Some("".to_string()));
            ytb_dl_info.set_file_path(Some("".to_string()));
            // 保存下载的视频地址
            ytb_dl_service.save_info(ytb_dl_info).await;

        },  
        Err(err) => println!("{:?}",err),
    }
}


pub async fn download_ytb_video_async()->Result<()>{
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



#[cfg(test)]
mod ytb_crawler_test {
    use super::*;
    use serde_json::json;
    use crate::crawler::YtbPlayerInfo;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on(async{$e})
        };
      }

    #[test]
    fn it_works() {
        aw!( get_ytb_info("rPAHwWfHcnE").await);
    }

    #[tokio::test]
    async fn test_channel_renderer() {
      let j =   json!(
        {
            "streamingData": {
                "expiresInSeconds": "21540",
                "formats": [
                    {
                        "itag": 17,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=17&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2F3gpp&gir=yes&clen=4902189&dur=525.746&lmt=1669827940656155&mt=1669870654&fvip=3&fexp=24001373%2C24007246&c=ANDROID&txp=5532434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIgeaI8ez-js87mh8QMum1LegXQOEhdjVq9GL7fmzMfgCwCIQCAgziKVradsyPMXQg_pxuk9UE7cDXe5fjt2JJHXmvj5w%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/3gpp; codecs=\"mp4v.20.3, mp4a.40.2\"",
                        "bitrate": 74606,
                        "width": 176,
                        "height": 144,
                        "lastModified": "1669827940656155",
                        "contentLength": "4902189",
                        "quality": "small",
                        "fps": 8,
                        "qualityLabel": "144p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 74594,
                        "audioQuality": "AUDIO_QUALITY_LOW",
                        "approxDurationMs": "525746",
                        "audioSampleRate": "22050",
                        "audioChannels": 1
                    },
                    {
                        "itag": 18,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=18&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fmp4&gir=yes&clen=46502410&ratebypass=yes&dur=525.676&lmt=1669827918299669&mt=1669870654&fvip=3&fexp=24001373%2C24007246&c=ANDROID&txp=5538434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cratebypass%2Cdur%2Clmt&sig=AOq0QJ8wRQIhAKKWCb_JPEC4CQAJTFsjHAsvlmpl-8Fgf99t-RbpVRZsAiB1qNbiwf6gbsYUJmA6EYRpvXXvZgGaFmMXtdBWin-0uA%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/mp4; codecs=\"avc1.42001E, mp4a.40.2\"",
                        "bitrate": 707765,
                        "width": 640,
                        "height": 360,
                        "lastModified": "1669827918299669",
                        "contentLength": "46502410",
                        "quality": "medium",
                        "fps": 30,
                        "qualityLabel": "360p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 707696,
                        "audioQuality": "AUDIO_QUALITY_LOW",
                        "approxDurationMs": "525676",
                        "audioSampleRate": "44100",
                        "audioChannels": 2
                    },
                    {
                        "itag": 22,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=22&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fmp4&cnr=14&ratebypass=yes&dur=525.676&lmt=1669832974258701&mt=1669870654&fvip=3&fexp=24001373%2C24007246&c=ANDROID&txp=5532434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Ccnr%2Cratebypass%2Cdur%2Clmt&sig=AOq0QJ8wRAIgIB8bSF8T4R3BjxEIjC3uuegnKSMlWjJCotrbfaEswjMCICWOKLuD2MVZTFFVDuW8XGhnqtjTREzrySkOJPP6lMiK&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/mp4; codecs=\"avc1.64001F, mp4a.40.2\"",
                        "bitrate": 1081997,
                        "width": 1280,
                        "height": 720,
                        "lastModified": "1669832974258701",
                        "quality": "hd720",
                        "fps": 30,
                        "qualityLabel": "720p",
                        "projectionType": "RECTANGULAR",
                        "audioQuality": "AUDIO_QUALITY_MEDIUM",
                        "approxDurationMs": "525676",
                        "audioSampleRate": "44100",
                        "audioChannels": 2
                    }
                ],
                "adaptiveFormats": [
                    {
                        "itag": 137,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=137&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fmp4&gir=yes&clen=222013595&dur=525.625&lmt=1669833283130190&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5535434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIgDR-eKypYhD5pRK0jy820KDYzYODWJOxI86BwnerUHecCIQDSA6-DphoLh8uUKfo9PpuuZsO1BjnF1xCktQDbkMkutg%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/mp4; codecs=\"avc1.640028\"",
                        "bitrate": 4565570,
                        "width": 1920,
                        "height": 1080,
                        "initRange": {
                            "start": "0",
                            "end": "740"
                        },
                        "indexRange": {
                            "start": "741",
                            "end": "2008"
                        },
                        "lastModified": "1669833283130190",
                        "contentLength": "222013595",
                        "quality": "hd1080",
                        "fps": 30,
                        "qualityLabel": "1080p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 3379041,
                        "approxDurationMs": "525625"
                    },
                    {
                        "itag": 248,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=248&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fwebm&gir=yes&clen=153556271&dur=525.624&lmt=1669832465863859&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5535434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIgIXwyVXNphFwVUnIeF1oU3QSrFqnGcAQyGNw-sKDq9B4CIQCP2QePD9PTnv0QrzrNv3vp-wO83Bk3F1-kn13hcLVssA%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/webm; codecs=\"vp9\"",
                        "bitrate": 2604779,
                        "width": 1920,
                        "height": 1080,
                        "initRange": {
                            "start": "0",
                            "end": "219"
                        },
                        "indexRange": {
                            "start": "220",
                            "end": "2049"
                        },
                        "lastModified": "1669832465863859",
                        "contentLength": "153556271",
                        "quality": "hd1080",
                        "fps": 30,
                        "qualityLabel": "1080p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 2337127,
                        "colorInfo": {
                            "primaries": "COLOR_PRIMARIES_BT709",
                            "transferCharacteristics": "COLOR_TRANSFER_CHARACTERISTICS_BT709",
                            "matrixCoefficients": "COLOR_MATRIX_COEFFICIENTS_BT709"
                        },
                        "approxDurationMs": "525624"
                    },
                    {
                        "itag": 399,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=399&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fmp4&gir=yes&clen=138376167&dur=525.625&lmt=1669829487402134&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5537434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIhAO2ZP0do2ej18SZEV0E1GQumL6PrTmKwnAaK_wUEtFGvAiBj_aPL8RMzJ_wW85NP3JOuSKcZ9iLTpu-1kbDX4sXNMA%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/mp4; codecs=\"av01.0.08M.08\"",
                        "bitrate": 3436355,
                        "width": 1920,
                        "height": 1080,
                        "initRange": {
                            "start": "0",
                            "end": "699"
                        },
                        "indexRange": {
                            "start": "700",
                            "end": "1967"
                        },
                        "lastModified": "1669829487402134",
                        "contentLength": "138376167",
                        "quality": "hd1080",
                        "fps": 30,
                        "qualityLabel": "1080p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 2106081,
                        "colorInfo": {
                            "primaries": "COLOR_PRIMARIES_BT709",
                            "transferCharacteristics": "COLOR_TRANSFER_CHARACTERISTICS_BT709",
                            "matrixCoefficients": "COLOR_MATRIX_COEFFICIENTS_BT709"
                        },
                        "approxDurationMs": "525625"
                    },
                    {
                        "itag": 136,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=136&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fmp4&gir=yes&clen=62612903&dur=525.625&lmt=1669832957407131&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5535434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIhAO_V8YmlxS8s6ERvreHSWeOOt7eYeeUFig1qaQkikZCiAiBfyRwfTxZ4CzTMjiiA1KX2UGD8K6TiohpCVpNTydKn1Q%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/mp4; codecs=\"avc1.4d401f\"",
                        "bitrate": 1748974,
                        "width": 1280,
                        "height": 720,
                        "initRange": {
                            "start": "0",
                            "end": "738"
                        },
                        "indexRange": {
                            "start": "739",
                            "end": "2006"
                        },
                        "lastModified": "1669832957407131",
                        "contentLength": "62612903",
                        "quality": "hd720",
                        "fps": 30,
                        "qualityLabel": "720p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 952966,
                        "approxDurationMs": "525625"
                    },
                    {
                        "itag": 247,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=247&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fwebm&gir=yes&clen=71605883&dur=525.624&lmt=1669833139170464&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5535434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRAIgUuuzFQGp2fHn2WjQuXdP3hSD_5P-4U2Hhd2nz9LPN4ECIG6iQ1I-pequCeMHoNpDVlrz8rX7BVHcSnTqGjG4Vf48&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/webm; codecs=\"vp9\"",
                        "bitrate": 1548397,
                        "width": 1280,
                        "height": 720,
                        "initRange": {
                            "start": "0",
                            "end": "219"
                        },
                        "indexRange": {
                            "start": "220",
                            "end": "2035"
                        },
                        "lastModified": "1669833139170464",
                        "contentLength": "71605883",
                        "quality": "hd720",
                        "fps": 30,
                        "qualityLabel": "720p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 1089841,
                        "colorInfo": {
                            "primaries": "COLOR_PRIMARIES_BT709",
                            "transferCharacteristics": "COLOR_TRANSFER_CHARACTERISTICS_BT709",
                            "matrixCoefficients": "COLOR_MATRIX_COEFFICIENTS_BT709"
                        },
                        "approxDurationMs": "525624"
                    },
                    {
                        "itag": 398,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=398&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fmp4&gir=yes&clen=68725810&dur=525.625&lmt=1669830402827869&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5537434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIhALCl3NfUPOfcVv7wo_idSD1SfYNn6Z5gJGt3aR_WrmsBAiA2pjYPggRlQL5mlF4DnYNE00pfY_8UTOOOdXBoS3oCKA%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/mp4; codecs=\"av01.0.05M.08\"",
                        "bitrate": 1542624,
                        "width": 1280,
                        "height": 720,
                        "initRange": {
                            "start": "0",
                            "end": "699"
                        },
                        "indexRange": {
                            "start": "700",
                            "end": "1967"
                        },
                        "lastModified": "1669830402827869",
                        "contentLength": "68725810",
                        "quality": "hd720",
                        "fps": 30,
                        "qualityLabel": "720p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 1046005,
                        "colorInfo": {
                            "primaries": "COLOR_PRIMARIES_BT709",
                            "transferCharacteristics": "COLOR_TRANSFER_CHARACTERISTICS_BT709",
                            "matrixCoefficients": "COLOR_MATRIX_COEFFICIENTS_BT709"
                        },
                        "approxDurationMs": "525625"
                    },
                    {
                        "itag": 135,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=135&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fmp4&gir=yes&clen=33916858&dur=525.625&lmt=1669833024865168&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5535434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIgaDvDkaTRoIgHGqh4_hNCaRkeNHuCJeO4Ly7UmcdcB58CIQDP5ehVtcivPt4xFB54-QzLc0Ae-k0iKNeqaDtYZG0thQ%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/mp4; codecs=\"avc1.4d401f\"",
                        "bitrate": 962420,
                        "width": 854,
                        "height": 480,
                        "initRange": {
                            "start": "0",
                            "end": "738"
                        },
                        "indexRange": {
                            "start": "739",
                            "end": "2006"
                        },
                        "lastModified": "1669833024865168",
                        "contentLength": "33916858",
                        "quality": "large",
                        "fps": 30,
                        "qualityLabel": "480p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 516213,
                        "approxDurationMs": "525625"
                    },
                    {
                        "itag": 244,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=244&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fwebm&gir=yes&clen=34556257&dur=525.624&lmt=1669833114119030&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5535434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIge7WnYz54zIeHuEaHj9LyXuSBcsUJCZ2rGqTxLloOJiwCIQCdPC4J0sCaES7AbreDUB6YMT8dgMw9sIcfBU4byc93Ug%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/webm; codecs=\"vp9\"",
                        "bitrate": 742615,
                        "width": 854,
                        "height": 480,
                        "initRange": {
                            "start": "0",
                            "end": "219"
                        },
                        "indexRange": {
                            "start": "220",
                            "end": "2008"
                        },
                        "lastModified": "1669833114119030",
                        "contentLength": "34556257",
                        "quality": "large",
                        "fps": 30,
                        "qualityLabel": "480p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 525946,
                        "colorInfo": {
                            "primaries": "COLOR_PRIMARIES_BT709",
                            "transferCharacteristics": "COLOR_TRANSFER_CHARACTERISTICS_BT709",
                            "matrixCoefficients": "COLOR_MATRIX_COEFFICIENTS_BT709"
                        },
                        "approxDurationMs": "525624"
                    },
                    {
                        "itag": 397,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=397&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fmp4&gir=yes&clen=36288525&dur=525.625&lmt=1669829542292728&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5537434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRgIhAIYcmcDTPAH51b7NOQd1SxURsyFUVbFPhavNHkZhT7clAiEAkQRTVBKkmi4uopARPxh_0ip6pFg5g75o2TYyc87ksPs%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/mp4; codecs=\"av01.0.04M.08\"",
                        "bitrate": 788900,
                        "width": 854,
                        "height": 480,
                        "initRange": {
                            "start": "0",
                            "end": "699"
                        },
                        "indexRange": {
                            "start": "700",
                            "end": "1967"
                        },
                        "lastModified": "1669829542292728",
                        "contentLength": "36288525",
                        "quality": "large",
                        "fps": 30,
                        "qualityLabel": "480p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 552310,
                        "colorInfo": {
                            "primaries": "COLOR_PRIMARIES_BT709",
                            "transferCharacteristics": "COLOR_TRANSFER_CHARACTERISTICS_BT709",
                            "matrixCoefficients": "COLOR_MATRIX_COEFFICIENTS_BT709"
                        },
                        "approxDurationMs": "525625"
                    },
                    {
                        "itag": 134,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=134&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fmp4&gir=yes&clen=21796831&dur=525.625&lmt=1669832954834319&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5535434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIhAOMLHbaHb9yXJYDsfD0rNTwjqhVvDYJ0NTO8kcHDFZh5AiARBIEUwA0hNHrfyb_QWiR1jaJ3otBHYiQ0voX-ODxniw%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/mp4; codecs=\"avc1.4d401e\"",
                        "bitrate": 563416,
                        "width": 640,
                        "height": 360,
                        "initRange": {
                            "start": "0",
                            "end": "738"
                        },
                        "indexRange": {
                            "start": "739",
                            "end": "2006"
                        },
                        "lastModified": "1669832954834319",
                        "contentLength": "21796831",
                        "quality": "medium",
                        "fps": 30,
                        "qualityLabel": "360p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 331747,
                        "highReplication": true,
                        "approxDurationMs": "525625"
                    },
                    {
                        "itag": 243,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=243&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fwebm&gir=yes&clen=23193769&dur=525.624&lmt=1669833120454219&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5535434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIhAOKYHuHDiR0rISP28rdDu_4_BzKuOOrCtMgj0-QJx3N1AiBwWkOFEqAb2xYILAzANsuD80MZe-qL1DLQKxqdaKcJ_A%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/webm; codecs=\"vp9\"",
                        "bitrate": 411005,
                        "width": 640,
                        "height": 360,
                        "initRange": {
                            "start": "0",
                            "end": "219"
                        },
                        "indexRange": {
                            "start": "220",
                            "end": "1985"
                        },
                        "lastModified": "1669833120454219",
                        "contentLength": "23193769",
                        "quality": "medium",
                        "fps": 30,
                        "qualityLabel": "360p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 353009,
                        "colorInfo": {
                            "primaries": "COLOR_PRIMARIES_BT709",
                            "transferCharacteristics": "COLOR_TRANSFER_CHARACTERISTICS_BT709",
                            "matrixCoefficients": "COLOR_MATRIX_COEFFICIENTS_BT709"
                        },
                        "approxDurationMs": "525624"
                    },
                    {
                        "itag": 396,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=396&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fmp4&gir=yes&clen=19787043&dur=525.625&lmt=1669828356611036&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5537434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRgIhAPgRhRrSF-bKls73r0XGUm1dlBTHQxumMm6OcrlmaS0rAiEAqEicp6ubzxhqjAA8VltQh94Jd1p9f7UR_h8tXI8E0-U%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/mp4; codecs=\"av01.0.01M.08\"",
                        "bitrate": 467357,
                        "width": 640,
                        "height": 360,
                        "initRange": {
                            "start": "0",
                            "end": "699"
                        },
                        "indexRange": {
                            "start": "700",
                            "end": "1967"
                        },
                        "lastModified": "1669828356611036",
                        "contentLength": "19787043",
                        "quality": "medium",
                        "fps": 30,
                        "qualityLabel": "360p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 301158,
                        "colorInfo": {
                            "primaries": "COLOR_PRIMARIES_BT709",
                            "transferCharacteristics": "COLOR_TRANSFER_CHARACTERISTICS_BT709",
                            "matrixCoefficients": "COLOR_MATRIX_COEFFICIENTS_BT709"
                        },
                        "approxDurationMs": "525625"
                    },
                    {
                        "itag": 133,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=133&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fmp4&gir=yes&clen=10890844&dur=525.625&lmt=1669832957228332&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5535434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIgNLHt0IaoSQaYD-5zEK8OpTaxMBvOW4OfKC_b1Su4-KoCIQCt2l5mTBtwCm1ddsOhP3hwBKWve-c0VycVhfsucK8kmw%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/mp4; codecs=\"avc1.4d4015\"",
                        "bitrate": 234522,
                        "width": 426,
                        "height": 240,
                        "initRange": {
                            "start": "0",
                            "end": "738"
                        },
                        "indexRange": {
                            "start": "739",
                            "end": "2006"
                        },
                        "lastModified": "1669832957228332",
                        "contentLength": "10890844",
                        "quality": "small",
                        "fps": 30,
                        "qualityLabel": "240p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 165758,
                        "approxDurationMs": "525625"
                    },
                    {
                        "itag": 242,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=242&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fwebm&gir=yes&clen=13070442&dur=525.624&lmt=1669833141417528&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5535434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIgKo67dWkVlpZ395Gjb0PN7Y7Wq1sfsE5gsKp8CQQhh7kCIQCBdmBa8B25ocG8fY1xmxvfccPnqSqkfhfqct1SqB38Ew%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/webm; codecs=\"vp9\"",
                        "bitrate": 225370,
                        "width": 426,
                        "height": 240,
                        "initRange": {
                            "start": "0",
                            "end": "218"
                        },
                        "indexRange": {
                            "start": "219",
                            "end": "1958"
                        },
                        "lastModified": "1669833141417528",
                        "contentLength": "13070442",
                        "quality": "small",
                        "fps": 30,
                        "qualityLabel": "240p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 198932,
                        "colorInfo": {
                            "primaries": "COLOR_PRIMARIES_BT709",
                            "transferCharacteristics": "COLOR_TRANSFER_CHARACTERISTICS_BT709",
                            "matrixCoefficients": "COLOR_MATRIX_COEFFICIENTS_BT709"
                        },
                        "approxDurationMs": "525624"
                    },
                    {
                        "itag": 395,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=395&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fmp4&gir=yes&clen=9801176&dur=525.625&lmt=1669828044468471&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5537434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIhAJboGBq17obB_7SeUtlLfYVJ444oXvGACsCMV9wyLCvlAiBY1TLzmC5H7A24rXwqT-rJpYcgMGCIvk4HHBZuAPWsCw%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/mp4; codecs=\"av01.0.00M.08\"",
                        "bitrate": 232866,
                        "width": 426,
                        "height": 240,
                        "initRange": {
                            "start": "0",
                            "end": "699"
                        },
                        "indexRange": {
                            "start": "700",
                            "end": "1967"
                        },
                        "lastModified": "1669828044468471",
                        "contentLength": "9801176",
                        "quality": "small",
                        "fps": 30,
                        "qualityLabel": "240p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 149173,
                        "colorInfo": {
                            "primaries": "COLOR_PRIMARIES_BT709",
                            "transferCharacteristics": "COLOR_TRANSFER_CHARACTERISTICS_BT709",
                            "matrixCoefficients": "COLOR_MATRIX_COEFFICIENTS_BT709"
                        },
                        "approxDurationMs": "525625"
                    },
                    {
                        "itag": 160,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=160&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fmp4&gir=yes&clen=5522003&dur=525.625&lmt=1669832968428261&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5535434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIgWqlFjSChRZDGb_K0MOh98RT5TUEBqm_dJtE-Kytjf-YCIQCFdLrwApm64ly5jXSGw7eKAznH66qMHAihhGav2js3vg%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/mp4; codecs=\"avc1.4d400c\"",
                        "bitrate": 105833,
                        "width": 256,
                        "height": 144,
                        "initRange": {
                            "start": "0",
                            "end": "737"
                        },
                        "indexRange": {
                            "start": "738",
                            "end": "2005"
                        },
                        "lastModified": "1669832968428261",
                        "contentLength": "5522003",
                        "quality": "tiny",
                        "fps": 30,
                        "qualityLabel": "144p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 84044,
                        "approxDurationMs": "525625"
                    },
                    {
                        "itag": 278,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=278&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fwebm&gir=yes&clen=6210013&dur=525.624&lmt=1669833113509611&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5535434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIhAL1L8-0kuwH95uJFE6WivhtHQaHU58SKN46AujKnr66xAiBF7fH4PeAXOW4c7J2xQF0Swlt-gO0VYapHB3g7DDLALg%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/webm; codecs=\"vp9\"",
                        "bitrate": 99396,
                        "width": 256,
                        "height": 144,
                        "initRange": {
                            "start": "0",
                            "end": "218"
                        },
                        "indexRange": {
                            "start": "219",
                            "end": "1957"
                        },
                        "lastModified": "1669833113509611",
                        "contentLength": "6210013",
                        "quality": "tiny",
                        "fps": 30,
                        "qualityLabel": "144p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 94516,
                        "colorInfo": {
                            "primaries": "COLOR_PRIMARIES_BT709",
                            "transferCharacteristics": "COLOR_TRANSFER_CHARACTERISTICS_BT709",
                            "matrixCoefficients": "COLOR_MATRIX_COEFFICIENTS_BT709"
                        },
                        "approxDurationMs": "525624"
                    },
                    {
                        "itag": 394,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=394&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=video%2Fmp4&gir=yes&clen=4968872&dur=525.625&lmt=1669827997858714&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5537434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIgfzSDADN-4PSI8qBAXkgQ2fSFyKj54KT6GXv5ZqJKWDMCIQCYqXLt7H33FmKgVPUS8eRVTjYNALYYwmhx55MBASt3FQ%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "video/mp4; codecs=\"av01.0.00M.08\"",
                        "bitrate": 101739,
                        "width": 256,
                        "height": 144,
                        "initRange": {
                            "start": "0",
                            "end": "699"
                        },
                        "indexRange": {
                            "start": "700",
                            "end": "1967"
                        },
                        "lastModified": "1669827997858714",
                        "contentLength": "4968872",
                        "quality": "tiny",
                        "fps": 30,
                        "qualityLabel": "144p",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 75626,
                        "colorInfo": {
                            "primaries": "COLOR_PRIMARIES_BT709",
                            "transferCharacteristics": "COLOR_TRANSFER_CHARACTERISTICS_BT709",
                            "matrixCoefficients": "COLOR_MATRIX_COEFFICIENTS_BT709"
                        },
                        "approxDurationMs": "525625"
                    },
                    {
                        "itag": 139,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=139&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=audio%2Fmp4&gir=yes&clen=3207094&dur=525.746&lmt=1669826353788650&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5532434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRgIhANcHfIaPaGxrxXM8Osl28ZHJ5Cw6jtjwXFmR4-uXzdeBAiEAvQHtVAyG7G7gAEZ1tH8P58_rS1qPKoGIxBMTHy02-Bg%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "audio/mp4; codecs=\"mp4a.40.5\"",
                        "bitrate": 50225,
                        "initRange": {
                            "start": "0",
                            "end": "640"
                        },
                        "indexRange": {
                            "start": "641",
                            "end": "1308"
                        },
                        "lastModified": "1669826353788650",
                        "contentLength": "3207094",
                        "quality": "tiny",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 48800,
                        "audioQuality": "AUDIO_QUALITY_LOW",
                        "approxDurationMs": "525746",
                        "audioSampleRate": "22050",
                        "audioChannels": 2
                    },
                    {
                        "itag": 140,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=140&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=audio%2Fmp4&gir=yes&clen=8508626&dur=525.676&lmt=1669826353399511&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5532434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRQIgctnkdUOHYkGZWQ9lnRIgxuONeBPP40uqKH7UJaA--bkCIQCuaGXDgEpEjzO4gS8s93gtJDwc4DZXrSF4cbItK9gl0A%3D%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "audio/mp4; codecs=\"mp4a.40.2\"",
                        "bitrate": 130720,
                        "initRange": {
                            "start": "0",
                            "end": "631"
                        },
                        "indexRange": {
                            "start": "632",
                            "end": "1299"
                        },
                        "lastModified": "1669826353399511",
                        "contentLength": "8508626",
                        "quality": "tiny",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 129488,
                        "highReplication": true,
                        "audioQuality": "AUDIO_QUALITY_MEDIUM",
                        "approxDurationMs": "525676",
                        "audioSampleRate": "44100",
                        "audioChannels": 2
                    },
                    {
                        "itag": 249,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=249&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=audio%2Fwebm&gir=yes&clen=3403552&dur=525.641&lmt=1669826508601447&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5532434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRAIgEZCIEXWmhG0HEQVdpj-FdIWxq8SaUB_3RZ8XfAFCu_MCIE3E8r1ao7y1QIFYg2GSQ2nDgZ1_8Dh3vmt2e0h_seFJ&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "audio/webm; codecs=\"opus\"",
                        "bitrate": 65407,
                        "initRange": {
                            "start": "0",
                            "end": "258"
                        },
                        "indexRange": {
                            "start": "259",
                            "end": "1156"
                        },
                        "lastModified": "1669826508601447",
                        "contentLength": "3403552",
                        "quality": "tiny",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 51800,
                        "audioQuality": "AUDIO_QUALITY_LOW",
                        "approxDurationMs": "525641",
                        "audioSampleRate": "48000",
                        "audioChannels": 2
                    },
                    {
                        "itag": 250,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=250&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=audio%2Fwebm&gir=yes&clen=4220576&dur=525.641&lmt=1669826508915195&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5532434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRgIhAJaiHHO21AYQ-aKmDxbimSuJ8dxiqjoDxyAqkP4l9r5JAiEA9s5gRn9sHWfFLNrtFS25s8hbBxGA-_4gzv_LqozzXJI%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "audio/webm; codecs=\"opus\"",
                        "bitrate": 83024,
                        "initRange": {
                            "start": "0",
                            "end": "258"
                        },
                        "indexRange": {
                            "start": "259",
                            "end": "1156"
                        },
                        "lastModified": "1669826508915195",
                        "contentLength": "4220576",
                        "quality": "tiny",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 64235,
                        "audioQuality": "AUDIO_QUALITY_LOW",
                        "approxDurationMs": "525641",
                        "audioSampleRate": "48000",
                        "audioChannels": 2
                    },
                    {
                        "itag": 251,
                        "url": "https://rr3---sn-oguelnzl.googlevideo.com/videoplayback?expire=1669892733&ei=HTaIY7ahL_202roPq6SY0A4&ip=50.7.158.186&id=o-AIE2gDhPSBUzq9fgP6EZa6PW8vM5axrj5TCeM0J_diMF&itag=251&source=youtube&requiressl=yes&mh=Do&mm=31%2C26&mn=sn-oguelnzl%2Csn-npoe7nsy&ms=au%2Conr&mv=m&mvi=3&pl=23&initcwndbps=1176250&vprv=1&mime=audio%2Fwebm&gir=yes&clen=7800804&dur=525.641&lmt=1669826509023809&mt=1669870654&fvip=3&keepalive=yes&fexp=24001373%2C24007246&c=ANDROID&txp=5532434&sparams=expire%2Cei%2Cip%2Cid%2Citag%2Csource%2Crequiressl%2Cvprv%2Cmime%2Cgir%2Cclen%2Cdur%2Clmt&sig=AOq0QJ8wRgIhAPS7hGFV6mCREnARCu7UyUGS3vmCvh70ZAWzjqgWFq7uAiEAzwzGYOLLByKnOkdTU5Ppa6eXEZEab0xDBKBUp0PLskM%3D&lsparams=mh%2Cmm%2Cmn%2Cms%2Cmv%2Cmvi%2Cpl%2Cinitcwndbps&lsig=AG3C_xAwRQIgJLmGGPr0x3vYsLNBQSvEmk22JPwAnzdvyZLmJCEl2W8CIQDxcun337wcpsNSpcn4NEdsY8cZ-aZwoUSCaWJggV77qQ%3D%3D",
                        "mimeType": "audio/webm; codecs=\"opus\"",
                        "bitrate": 153906,
                        "initRange": {
                            "start": "0",
                            "end": "258"
                        },
                        "indexRange": {
                            "start": "259",
                            "end": "1156"
                        },
                        "lastModified": "1669826509023809",
                        "contentLength": "7800804",
                        "quality": "tiny",
                        "projectionType": "RECTANGULAR",
                        "averageBitrate": 118724,
                        "audioQuality": "AUDIO_QUALITY_MEDIUM",
                        "approxDurationMs": "525641",
                        "audioSampleRate": "48000",
                        "audioChannels": 2
                    }
                ]
            },
            "videoDetails": {
                "videoId": "rPAHwWfHcnE",
                "title": "出大事了！廣州突然宣布解封，大家都懵了，防疫人員突然失業很失落。抗議奏效還是因江澤民死了？廣州新增逾六千病例居全國之首突然解封而北京卻大舉建造新的巨大方艙隔離點。廣州解封｜廣州疫情｜北京疫情｜中國疫情",
                "lengthSeconds": "526",
                "channelId": "UCGPbBlQeLIJF1XqVkYmEV6g",
                "isOwnerViewing": false,
                "shortDescription": "已開啟會員功能，希望能得到您的支持，讓更多人看到中國真相。感謝！！！：\nhttps://www.youtube.com/channel/UCGPbBlQeLIJF1XqVkYmEV6g/join\n\n紀錄真實的中國，希望這片神奇的土地能夠正常一些。歡迎大家訂閱，謝謝！\n\n本頻道接受投稿啦！只要您覺得您的素材符合本頻道的調性，讓更多人看到真實的中國，那麼請您聯繫我：https://t.me/zaiyeshuo\n\n邀請你加入“在野說Telegram交流群”暢所欲言：https://t.me/zaiyeshuoqun\n\n邀請你關注在野說的twitter：https://twitter.com/zaiyeshuo",
                "isCrawlable": true,
                "thumbnail": {
                    "thumbnails": [
                        {
                            "url": "https://i.ytimg.com/vi/rPAHwWfHcnE/default.jpg",
                            "width": 120,
                            "height": 90
                        },
                        {
                            "url": "https://i.ytimg.com/vi/rPAHwWfHcnE/mqdefault.jpg",
                            "width": 320,
                            "height": 180
                        },
                        {
                            "url": "https://i.ytimg.com/vi/rPAHwWfHcnE/hqdefault.jpg",
                            "width": 480,
                            "height": 360
                        },
                        {
                            "url": "https://i.ytimg.com/vi/rPAHwWfHcnE/sddefault.jpg",
                            "width": 640,
                            "height": 480
                        }
                    ]
                },
                "allowRatings": true,
                "viewCount": "622499",
                "author": "在野說",
                "isPrivate": false,
                "isUnpluggedCorpus": false,
                "isLiveContent": false
            }
        }
        );
        let u: YtbPlayerInfo = serde_json::from_value(j).unwrap();
        println!("{:?}",u);
    }

}
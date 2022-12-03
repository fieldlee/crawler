#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]
#[macro_use]
extern crate getset;
#[macro_use]
extern crate rbatis;


//配置
pub mod config;
//初始化
pub mod init;
//orm
pub mod orm;
//cache
pub mod cache;
//utils
pub mod utils;
//models
pub mod model;
// services
pub mod services;
// crud
pub mod crud;
//crawler
pub mod crawler;

use log::info;
use state::Container;
use crawler::ytb_crawler::ytb_crawler_youtube3;
use crawler::ytb_download::get_ytb_info;
use crawler::ytb_download::download_ytb_video_async;
use utils::error::Result;
use crate::services::crawler_service::CrawlerService;
/*
    整个项目上下文ApplicationContext
    包括:
        ApplicationConfig 配置
        Database mongodb数据库
        Rbatis  mysql orm
        ServiceContext 服务上下文
        CasbinService 权限服务
*/

use init::init_config;
use init::init_log;
use init::init_database;
use config::config::ApplicationConfig;

use crate::init::init_service;

pub static APPLICATION_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();


/*初始化环境上下文*/
pub async fn init_context() {
     print_banner();
     //第一步加载配置
     init_config().await;
     //第二步加载日志
     init_log();
     info!("ConfigContext init complete");
     //第三步初始化数据源
     init_database().await;
     //第四步
     init_service().await;

    info!("DataBase init complete");
    let commerce_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //第五步
    // tokio::spawn(start_crawler());

    let crawler_service = APPLICATION_CONTEXT.get::<CrawlerService>();

    let crawler_list = crawler_service.get_crawler_list().await;

    println!("{:?}", crawler_list);
}

pub async fn start_crawler() -> Result<()> {
    let result  = ytb_crawler_youtube3("美女").await?;

    Ok(())
}

pub async fn start_get_ytb_info() -> Result<()> {
    let list = vec!["Mbz7wvVdT2E","l2SxVulhgmA","204klcjBia0","_7DbZ4PC90g","j_FZdnts2fE"];
    for item in list{
        let result  = get_ytb_info(item).await?;
    }
    Ok(())
}


pub async fn start_download() -> Result<()> {
    let result  = download_ytb_video_async().await?;
    Ok(())
}

fn print_banner() {
    let banner = r#"
     ____
    |      。   ———————     |                |     |        _____    ____
    |___   |   |            |                |     |       |        |
    |      |   |_______     |         —————— |     |       |_____   |____
    |      |   |            |         |      |     |       |        |
    |      |   |————————    |______   |_____ |     |____   |_____   |____
"#;
    println!("{}", banner);
}

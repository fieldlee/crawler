use thirtyfour::prelude::*;
use crate::utils::error::Result;
use thirtyfour::WebDriver;



pub mod ytb_crawler;
pub mod ytb_crawler_with_swas;
pub mod ytb_download;

pub async fn create_thirtyfour()->Result<WebDriver>{
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    Ok(driver)
}
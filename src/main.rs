use tokio::task;
use crawler::{
    init_context,
    start_download,
    start_crawler,
    start_get_ytb_info,
};

#[tokio::main]
async fn main() {
    //初始化上环境下文
    init_context().await;

    let _result = start_download().await;
    
}


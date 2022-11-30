
use crawler::{
    init_context,
    start_crawler,
};

#[tokio::main]
async fn main() {
    //初始化上环境下文
    init_context().await;
    let result = start_crawler().await;
    println!("start_crawler：{:?}", result);

}


use crate::config::config::ApplicationConfig;
use rbatis::rbatis::Rbatis;


///实例化 rbatis orm 连接池
pub async fn init_rbatis(commerce_config: &ApplicationConfig) -> Rbatis {
    let rbatis = Rbatis::new();
    if commerce_config.debug().eq(&false) && rbatis.is_debug_mode() {
        panic!(
            r#"已使用release模式，但是rbatis仍使用debug模式！请删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#
        );
    }
    //连接数据库
    println!(
        "rbatis link database ({})...",
        commerce_config.database_url().clone()
    );
    rbatis
        .link(&commerce_config.database_url())
        .await
        .expect("rbatis link database fail!");
    println!("rbatis link database success!");

    return rbatis;
}

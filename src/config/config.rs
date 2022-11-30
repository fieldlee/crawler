
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone, Getters, Setters)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct OSSConfig {
    key_id: String,
    key_secret: String,
    endpoint: String,
    bucket: String,
    access_endpoint: String,
}
impl OSSConfig {
    pub fn validate(&self) {
        if self.key_id.is_empty()
            || self.key_secret.is_empty()
            || self.endpoint.is_empty()
            || self.bucket.is_empty()
        {
            panic!("请配置oss ！！！！！！！！！！！！！！！！！！！")
        }
    }
}
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone)]
pub enum UploadType {
    OSS,
}
///服务启动配置
#[derive(
    Debug, PartialEq, serde::Serialize, serde::Deserialize, Clone, Getters, Setters, MutGetters,
)]
#[getset(get_mut = "pub", get = "pub", set = "pub")]
pub struct ApplicationConfig {
    debug: bool,
    is_demo: bool,
    ///redis地址
    redis_url: String,
    /// 数据库地址
    database_url: String,
    /// 逻辑删除字段
    logic_column: String,
    logic_un_deleted: i64,
    logic_deleted: i64,
    ///日志目录 "target/logs/"
    log_dir: String,
    /// "100MB" 日志分割尺寸-单位KB,MB,GB
    log_temp_size: String,
    /// 日志打包格式可选“”（空-不压缩）“gzip”（gz压缩包）“zip”（zip压缩包）“lz4”（lz4压缩包（非常快））
    log_pack_compress: String,
    ///日志滚动配置   保留全部:All,按时间保留:KeepTime(Duration),按版本保留:KeepNum(i64)
    log_rolling_type: String,
    ///日志等级
    log_level: String,
    ///api 秘钥
    api_key: String,
    ///权限缓存类型
    cache_type: String,
    upload_type: UploadType,
    //oss 配置
    oss: OSSConfig,
}

impl ApplicationConfig {
    pub fn new(yml_data: &str) -> Self {
        let config = match serde_yaml::from_str(yml_data) {
            Ok(e) => e,
            Err(e) => panic!("{}", e),
        };
        config
    }
    pub fn validate(&self) {
        if self.redis_url.is_empty() {
            panic!("请配置redis_url ！！！！！！！！！！！！！！！！！！！")
        }
        if self.database_url.is_empty() {
            panic!("请配置database_url ！！！！！！！！！！！！！！！！！！！")
        }
    }
}

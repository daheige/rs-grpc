use crate::infras::config::{Config, ConfigTrait};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::Path;

// AppConfig 项目配置信息
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub app_name: String,
    pub app_debug: bool,
    pub grpc_port: u16,
    pub monitor_port: u16,
    // 其他配置可自行配置
}

// config read and init app config
pub static APP_CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    let config_dir = std::env::var("CONFIG_DIR").unwrap_or("./".to_string());
    let filename = Path::new(config_dir.as_str()).join("app.yaml");
    println!("filename:{:?}", filename);

    let c = Config::load(filename);
    // read config to struct
    let conf: AppConfig = serde_yaml::from_str(c.content()).unwrap();
    // 开发过程中，可以取消下面的注释
    if conf.app_debug {
        println!("conf:{:?}", conf);
    }

    conf
});

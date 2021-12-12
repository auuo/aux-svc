use std::env;

use config::Config;
use lazy_static::lazy_static;

lazy_static! {
    /// 加载配置文件。通过环境变量 config_dir 指定配置文件路径，默认 conf。通过环境变量 profile 指定配置文件。
    /// 配置文件格式：application_{profile}.[yaml|json|toml]
    ///
    /// # Example
    ///
    /// ```
    /// use aux_svc::APP_CONFIG;
    /// println!("{}", APP_CONFIG.get_str("mysql.url").unwrap());
    /// ```
    pub static ref APP_CONFIG: Config = {
        let mut _config = Config::new();
        let config_dir = env::var("config_dir").unwrap_or("conf".to_string());
        _config.merge(config::File::with_name(&format!("{}/application", config_dir))).unwrap();

        if let Ok(profile) = env::var("profile") {
            let config_file_name = format!("{}/application_{}", config_dir, profile);
            _config.merge(config::File::with_name(&config_file_name)).unwrap();
        }

        _config
    };
}

/// 用于定义配置项
///
/// # Example
///
/// ```
/// aux_svc::config_keys! {
///     (MYSQL_URL, "mysql.url");
/// }
/// println!("{}", ConfigKey::MYSQL_URL);
/// ```
#[macro_export] macro_rules! config_keys {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $key:expr);
        )+
    ) => {
        pub struct ConfigKey;

        impl ConfigKey {
        $(
            $(#[$docs])*
            pub const $name: &'static str = $key;
        )+
        }
    }
}

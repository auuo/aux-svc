use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use anyhow::anyhow;
pub use fluent::FluentArgs as Args;
use fluent::FluentResource;
use fluent_bundle::bundle::FluentBundle;
use intl_memoizer::concurrent::IntlLangMemoizer;
use lazy_static::lazy_static;

lazy_static! {
    static ref BUNDLES_MAP: HashMap<String, FluentBundle<FluentResource, IntlLangMemoizer>> = {
        init().unwrap()
    };
}

/// 获取 i18n 信息
pub fn get_message(lang: impl Into<String>, key: &str, args: Option<&Args>) -> Option<String> {
    if let Some(bundle) = BUNDLES_MAP.get(&lang.into()) {
        if let Some(msg) = bundle.get_message(key) {
            if let Some(pattern) = msg.value() {
                let value = bundle.format_pattern(pattern, args, &mut vec![]);
                return Some(value.to_string());
            }
        }
    }
    None
}

fn init() -> anyhow::Result<HashMap<String, FluentBundle<FluentResource, IntlLangMemoizer>>> {
    let mut bundles = HashMap::new();
    let resource = get_available_resource(env::var("i18n_dir").unwrap_or("i18n/".to_owned()))?;
    for (langid, source) in resource.iter() {
        let mut bundle = FluentBundle::new_concurrent(vec![langid.parse()?]);
        bundle.add_resource(FluentResource::try_new(source.clone())
            .map_err(|e| anyhow!("parse resource error: {:?}", e))?)
            .map_err(|e| anyhow!("add resource error: {:?}", e))?;
        bundles.insert(langid.clone(), bundle);
    }
    Ok(bundles)
}

fn get_available_resource<P: AsRef<Path>>(dir: P) -> anyhow::Result<HashMap<String, String>> {
    let mut result = HashMap::new();
    let res_dir = fs::read_dir(dir)?;
    for entry in res_dir {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                continue;
            }
            if let Some(name) = path.file_name() {
                if let Some(name) = name.to_str() {
                    if name.starts_with("messages_") && name.ends_with(".ftl") {
                        let langid = name.trim_start_matches("messages_")
                            .trim_end_matches(".ftl")
                            .to_owned();
                        result.insert(langid, read_file(&path)?);
                    }
                }
            }
        }
    }
    Ok(result)
}

fn read_file(path: &Path) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
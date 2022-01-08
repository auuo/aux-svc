# aux-svc
后端服务通用组件实现，包括：
- 配置文件读取及配置项配置
- 枚举项定义，支持 `&'static str` 和 `i8` 互转，主要用于数据库存储 `i8` 前端返回字符串
- 错误项定义
- i18n 信息
- 常用上下文信息，如用户语言、追踪使用的 log id

## 快速开始
**1. 新建项目**

```shell
cargo new blog
```

**2. 增加配置文件**

在项目根目录新增 conf 文件夹用于存放配置文件：
```shell
mkdir conf
cd conf
touch application.yaml # 该文件必须存在，可为空。若指定了 profile 则优先使用 profile 配置文件
touch application_dev.yaml
touch application_prod.yaml
```

启动任务时可使用环境变量 `profile=dev` 指定要使用的配置文件。若不想使用 conf 作为配置文件夹名，可使用 config_dir 环境变量指定自定义路径

**3. 定义配置项**

在项目 lib.rs 文件中定义配置项，对应配置文件中的配置项：

```rust
pub mod config {
    pub use aux_config::APP_CONFIG;

    aux_config::config_keys! {
        pub ConfigKey {
            (MYSQL_URL, "mysql.url");
        }
    }
}
```

定义之后可以在程序任意地方读取配置：

```rust
fn db_url() -> String {
    use aux_config::APP_CONFIG;
    APP_CONFIG.get_str(ConfigKey::MYSQL_URL).unwrap()
}
```

**4. 增加错误类**

在项目 lib.rs 文件中定义业务错误：

```rust
pub mod error {
    aux_error::define_error! {
        pub AppError {
            MissBody(1, "miss_body");
            InvalidBlogLength(2, "invalid_blog_length");
        }
    }
}
```

括号中数字为错误码，字符串为 i18n 的 key。定义的错误除了用户显示定义的外还存在一个 `AppError::Unknown(anyhow::Error)` 用于包装其他所有类型的错误。定义错误项后可在程序中返回错误：

```rust
fn create_blog(blog: Blog) -> Result<(), AppError> {
    if blog.name.is_none() {
        return Err(AppError::MissBody(None));
    }
    if blog.content.len() > 128 {
        return Err(AppError::InvalidBlogLength(Some(fluent::fluent_args![ // 传递错误参数信息
            "len" => blog.content.len(),
        ])))
        // i18n 文件中：invalid_blog_length = 博文最大长度 128，你的长度为 { $len } 
    }
    Ok(db.save(blog)?) // 未知错误会包装为 AppError::Unknown
}
```

**5. 配置错误 i18n**

创建 i18n 文件夹，并在其中创建需要支持的语言信息文件：

```shell
mkdir i18n
cd i18n
touch messages_en_US.ftl
touch messages_zh_CN.ftl
```

若不想使用 i18n 作为文件夹名，可使用环境变量 `i18n_dir` 指定自定义文件夹路径。`messages_zh_CN.ftl` 内容演示如下：

```text
miss_body = 缺少必填参数
invalid_blog_length = 博文最大长度 128，你的长度为 { $len } 
```

除了在错误项中已配置的 key 外也可增加自定义 key。接着可以在任意地方使用：

```rust
fn show_msg() {
    use aux_i18n::get_message;
    
    let lang = "zh_CN";
    let msg = get_message(lang, "miss_body", None).unwrap();
    println("{}", msg);
}
```

最佳实践是定义中间件，在请求开始解析用户语言，存入 aux_context 的 LANG 中，在请求结束后取出 LANG 和 i18n key 获取对应消息进行返回。

**6. 增加业务枚举**

在 lib.rs 中定义业务枚举:

```rust
pub mod enums {
    aux_enums::enums! {
        pub BlogVisibility {
            Open(0, "open");
            OnlySelf(1, "onlySelf");
        }
    }
}
```

一些使用姿势：

```rust
fn use_enums() {
    let save_as_i8 = BlogVisibility::Open.num(); // 一般用于存入数据库
    let return_as_str = BlogVisibility::Open.alias(); // 一般用于返回给前端，看喜好，有些返回前端也喜欢用数字，省去一步转换
    
    let from_i8 = BlogVisibility::num_of(1).unwrap().alias(); // 从数据库的 i8 转前端的字符串
    let from_str = BlogVisibility::alias_of("open").unwrap().num(); // 从前端用户输入转数据库数字
}
```

**7. 上下文变量**

用来存放一些通用的信息，目前支持 LANG 和 LOG_ID。LOG_ID 可使用 `aux_logid` crate 生成。最佳实践是在请求开始解析 LANG 和生成 LOG_ID，以 `axum` 框架为例子创建中间件：

```rust
use std::task::{Context, Poll};

use axum::http::{Request, Response};
use futures::future::BoxFuture;
use tower::Service;

#[derive(Clone)]
pub struct ContextMiddleware<S> {
    pub inner: S,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for ContextMiddleware<S>
    where
        S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
        S::Future: Send + 'static,
        ReqBody: Send + 'static,
        ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        use aux_context::{LANG, LOG_ID};

        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            let lang = req.headers().get("Accept-Language")
                .and_then(|it| it.to_str().ok())
                .unwrap_or("")
                .to_owned();
            let log_id = req.headers().get("x-log-id") // 从请求中解析 log id，若不存在则生成一个新的
                .and_then(|it| it.to_str().ok())
                .map(|it| it.to_string())
                .unwrap_or(aux_logid::gen_log_id());
            Ok(LANG.scope(lang,
                LOG_ID.scope(log_id, inner.call(req))
            ).await?)
        })
    }
}
```

为 axum 的 router 添加 layer，使得使用 trace 打印日志时添加 log id 用于串联日志：

```rust
// Router::new()
//      .layer(TraceLayer::new_for_http().make_span_with(TraceMakeSpan {}))
//      .layer(layer_fn(|inner| ContextMiddleware { inner }))

#[derive(Debug, Clone)]
struct TraceMakeSpan;

impl<B> MakeSpan<B> for TraceMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let log_id = aux_context::LOG_ID.with(|it| it.clone());
        tracing::span!(
            Level::DEBUG,
            "request",
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
            traceID = %log_id,
        )
    }
}
```
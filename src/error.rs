use anyhow::Error;
use thiserror::Error;

/// 包装业务异常与未知异常
#[derive(Error, Debug)]
pub enum AppError {
    #[error("todo")] // todo
    Application((i64, &'static str, &'static str), Option<anyhow::Error>),

    #[error("unknown error, {0}")]
    Unknown(anyhow::Error),
}

impl From<anyhow::Error> for AppError {
    fn from(err: Error) -> Self {
        Self::Unknown(err)
    }
}

impl AppError {
    pub fn new(code: (i64, &'static str, &'static str)) -> Self {
        Self::Application(code, None)
    }

    pub fn new_with_err(code: (i64, &'static str, &'static str), err: anyhow::Error) -> Self {
        Self::Application(code, Some(err))
    }
}

/// 定义业务异常
///
/// # Example
///
/// ```
/// aux_svc::err_codes! {
///    (INVALID_PARAMS, "wrong params, please check again", "invalid.params");
/// }
/// ```
#[macro_export] macro_rules! err_codes {
    (
        $(
            $(#[$docs:meta])*
            ($name:ident, $code:expr, $msg:expr, $msg_key:expr);
        )+
    ) => {
        pub struct ErrCode;

        impl ErrCode {
        $(
            $(#[$docs])*
            pub const $name: (i64, &'static str, &'static str) = ($code, $msg, $msg_key);
        )+
        }
    }
}

use anyhow::Error;
use thiserror::Error;

/// 包装业务异常与未知异常
#[derive(Error, Debug)]
pub enum AppError {
    #[error("{}", (.0).1)]
    Application((i64, &'static str, &'static str), Option<anyhow::Error>, Option<crate::i18n::Args<'static>>),

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
        Self::Application(code, None, None)
    }

    pub fn new_with_err(code: (i64, &'static str, &'static str), err: anyhow::Error) -> Self {
        Self::Application(code, Some(err), None)
    }

    pub fn new_with_args(code: (i64, &'static str, &'static str), args: crate::i18n::Args<'static>) -> Self {
        Self::Application(code, None, Some(args))
    }

    pub fn new_with_err_and_args(code: (i64, &'static str, &'static str), err: anyhow::Error, args: crate::i18n::Args<'static>) -> Self {
        Self::Application(code, Some(err), Some(args))
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

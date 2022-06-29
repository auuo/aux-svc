/// 定义错误类型，携带 code、msg 和一个自定义类型的数据
///
/// # Example
///
/// ```
/// aux_svc::define_error! {
///     // 可用于传递 i18n 参数
///     pub AppError<aux_i18n::Args<'static>> {
///         MissBody(1, "miss_body");
///         InvalidParams(2, "invalid_params");
///     }
/// }
///
/// println!(AppError::MissBody(None)); // 无 i18n 参数
/// println!(AppError::InvalidParams(Some(fluent::fluent_args![ // 传递 i18n 参数
///     "field" => "name",
/// ])))
/// ```
#[macro_export] macro_rules! define_error {
    (
        $(#[$docs:meta])*
        $vis:vis $enum_name:ident<$args_type:ty> {
            $($name:ident($code:expr, $msg:expr);)+
        }
    ) => {
        $(#[$docs])*
        #[derive(Debug, thiserror::Error)]
        $vis enum $enum_name {
            $(
                #[error("{}", stringify!($name))]
                $name(Option<$args_type>),
            )+

            #[error(transparent)]
            Unknown(#[from] anyhow::Error),
        }

        impl $enum_name {
            pub fn get_code(&self) -> Option<i32> {
                match self {
                    $($enum_name::$name(..) => Some($code),)+
                    $enum_name::Unknown(..) => None,
                }
            }

            pub fn get_msg(&self) -> Option<&'static str> {
                match self {
                    $($enum_name::$name(..) => Some($msg),)+
                    $enum_name::Unknown(..) => None,
                }
            }

            pub fn get_args(&self) -> Option<&$args_type> {
                match self {
                    $($enum_name::$name(a) => a.as_ref(),)+
                    $enum_name::Unknown(..) => None,
                }
            }
        }
    }
}

/// 用于定义可以 into 到 Unknown(anyhow::Error) 的类型
///
/// # Example
///
/// ```
/// aux_error::define_error_from! {
///     AppError(reqwest::Error) // 定义可从 reqwest::Error 类型转换到 Unknown 类型
/// }
///
/// reqwest::get().await? // 可使用 ? 直接向上返回错误
/// ```
#[macro_export] macro_rules! define_error_from {
    ($enum_name:ident($from_ty:ty);) => {
        impl From<$from_ty> for $enum_name {
            fn from(e: $from_ty) -> Self {
                $enum_name::Unknown(e.into())
            }
        }
    }
}
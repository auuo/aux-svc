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

            #[error("unknown error, {0}")]
            Unknown(anyhow::Error),
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

        impl From<anyhow::Error> for $enum_name {
            fn from(err: anyhow::Error) -> Self {
                Self::Unknown(err)
            }
        }
    }
}

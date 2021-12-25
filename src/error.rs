#[macro_export] macro_rules! define_error {
    (
        $(#[$docs:meta])*
        $vis:vis $enum_name:ident {
            $($name:ident($code:expr, $msg_key:expr);)+
        }
    ) => {
        $(#[$docs])*
        #[derive(Debug, thiserror::Error)]
        $vis enum $enum_name {
            $(
                #[error("$name")]
                $name(Option<$crate::i18n::Args<'static>>),
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

            pub fn get_msg_key(&self) -> Option<&'static str> {
                match self {
                    $($enum_name::$name(..) => Some($msg_key),)+
                    $enum_name::Unknown(..) => None,
                }
            }

            pub fn get_i18n_args(&self) -> Option<&$crate::i18n::Args<'static>> {
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

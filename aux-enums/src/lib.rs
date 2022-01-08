/// 定义枚举数据，在 &'static str 与 i8 之间转换
///
/// # Example
///
/// ```
/// aux_svc::enums! {
///     pub TaskStatus {
///         Open(0, "open");
///         Close(1, "close");
///     }
/// }
///
/// println!(TaskStatus::Open.alias());
/// println!(TaskStatus::Close.num());
/// println!(TaskStatus::alias_of("open").unwrap());
/// println!(TaskStatus::num_of(1).unwrap());
/// ```
#[macro_export] macro_rules! enums {
    (
        $(
            $(#[$docs:meta])*
            $vis:vis $enum_name:ident {
                $($name:ident($num:expr, $alias:expr);)+
            }
        )+
    ) => {
        $(
            $(#[$docs])*
            #[derive(Debug, Clone, Copy, Eq, PartialEq)]
            #[repr(i8)]
            $vis enum $enum_name {
                $($name = $num,)+
            }

            impl $enum_name {
                pub fn alias(&self) -> &'static str {
                    match self {
                        $($enum_name::$name => $alias,)+
                    }
                }

                pub fn num(&self) -> i8 {
                    *self as i8
                }

                pub fn alias_of(alias: impl Into<String>) -> Option<Self> {
                    match alias.into().as_str() {
                        $($alias => Some($enum_name::$name),)+
                        _ => None
                    }
                }

                pub fn num_of(num: i8) -> Option<Self> {
                    match num {
                        $($num => Some($enum_name::$name),)+
                        _ => None
                    }
                }
            }
        )+
    }
}

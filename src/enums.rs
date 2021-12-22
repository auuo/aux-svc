/// 定义枚举数据，在 &'static str 与 i8 之间转换
///
/// # Example
///
/// ```
/// aux_svc::enums! {
///     TaskStatus {
///         (OPEN, "open", 0);
///         (CLOSE, "close", 1);
///     }
/// }
///
/// println!(TaskStatus::OPEN.get_alias());
/// println!(TaskStatus::OPEN.get_num());
/// println!(TaskStatus::alias_of("open").unwrap());
/// println!(TaskStatus::num_of(1).unwrap());
/// ```
#[macro_export] macro_rules! enums {
    (
        $(
            $(#[$docs:meta])*
            $enum_name:ident {
                $(($name:ident, $alias:expr, $num:expr);)+
            }
        )+
    ) => {
        $(
            $(#[$docs])*
            #[derive(Debug, Clone, Copy, Eq, PartialEq)]
            #[repr(i8)]
            pub enum $enum_name {
                $($name = $num,)+
            }

            impl $enum_name {
                pub fn get_alias(&self) -> &'static str {
                    match self {
                        $($enum_name::$name => $alias,)+
                    }
                }

                pub fn get_num(&self) -> i8 {
                    self as i8
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

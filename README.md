# aux-svc
后端服务通用组件实现，包括：
- 配置文件读取及配置项配置
- 枚举项定义，支持 `&'static str` 和 `i8` 互转，主要用于数据库存储 `i8` 前端返回字符串
- 错误项定义
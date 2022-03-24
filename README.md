## rust warp 版 crud

rust 真是一门晦涩难搞的语言，关键是文档资料搬砖代码着实是少了些，虽然库很丰富，但是教程案例要么简单要么难懂。各种 google，各种 bing。最后还得各种试，搬砖体验极差。对着一句代码挣扎半天的情况时有发生。本着为人民服务的思维，把最近捣鼓的结果共享下，稍微优化下 rust 后来者的搬砖体验。

## 人狠话不多

先上代码

https://github.com/wesin/harry-crud.git

## warp 版 web 服务端

为什么选择 warp。只是因为 warp 的使用案例比较容易看懂，什么 axum 案例文档更少，靠猜写代码真他妈太难了。

## 数据库使用的 postgresql

用就完事了

## sea-orm 脱离 sql 写代码

增加搬砖体验吧，写 sql 无疑增加了代码量，而且容易出错。简单功能简单实现，怎么快怎么来。

## 本项目知识点（搬砖用）

- once_cell 单例用法 涉及数据连接池对象 db_center、程序配置对象 config
- log4rs 配置及用法
- thiserror 错误统一处理的封装和用法
- config 项目配置的使用
- warp web 框架
- sea-orm orm 数据库框架
- sea-orm-cli 运行项目的 build.bat 自动生成 sea-orm 对应的数据库操作类。
- 文件上传下载
- 问号在 warp crud 代码中的使用，简化 match 错误处理代码

## 怎么玩

1. 下载项目
2. resource 文件夹里有建库建表 sql
3.

```
cargo run
```

4. resource 里还有 postman 导出的接口测试

## see you

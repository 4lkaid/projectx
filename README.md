# ProjectX

`ProjectX`是一个基于[axum](https://github.com/tokio-rs/axum)框架的通用模板。

## 概述

ProjectX是一个web应用模板，旨在为使用Rust开发web应用提供有价值的参考。该模板遵循特定的项目结构约定，以确保开发过程的标准化和可扩展性。

## 使用指南

1. 克隆仓库。
2. 根据需要自定义`.env`和`config.toml`文件中的配置。
3. 执行数据库迁移。

   ```bash
   # 确保已安装`sqlx-cli`。可以使用以下命令进行安装：
   cargo install sqlx-cli

   # 确保`.env`文件中配置了正确的数据库连接信息。
   DATABASE_URL=postgres://username:password@host/database

   # 在DATABASE_URL处创建数据库：
   sqlx database create

   # 执行数据迁移命令以设置数据库表：
   sqlx migrate run
   ```

4. 使用`cargo run`启动应用程序。
5. 通过指定的地址访问应用程序，并使用提供的API功能。

## 最低支持Rust版本

ProjectX支持的最低Rust版本为1.70。

## 许可协议

本项目使用[MIT许可证](LICENSE)授权。

# Nonebot-rs

<a href="https://github.com/botuniverse/onebot/blob/master/v11/specs/README.md">
  <img src="https://img.shields.io/badge/OneBot-v11-black">
</a>
<a href="https://github.com/abrahum/nonebot-rs/blob/master/license">
  <img src="https://img.shields.io/github/license/abrahum/nonebot-rs" alt="license">
</a>
<a href="https://crates.io/crates/nonebot_rs">
  <img src="https://img.shields.io/crates/v/nonebot_rs">
</a>

**A Onebot SDK in Rust**

~~计划是实现 Nonebot2 的完整架构~~

一个基础功能完备的可扩展 Onebot SDK ，使用 Plugin 作为扩展。nbrs 本体负责与 Onebot 实现端建立连接、将 Onebot 通信转化抽象为 Event 与 Bot (可以调用 Onebot Api 的 struct)，并向各 Plugin 分发、读取配置文件。

目前作为 feature 内建有 matcher (与 Nonebot 类似机制的匹配处理机制)、scheduler (定时任务) Plugin。(其实 logger 也是一个内建插件)。

目前已经有计划的 Plugin 有: nbrs_lua(lua)、nbrs_py(Python)。

API文档地址：[Docs.rs](https://docs.rs/nonebot_rs/)

## 各项目简介

- nonebot_rs: nbrs 本体
- nbrs_no4: nbrs 实例项目
- nbrs_lua: 使用 lua 为 nbrs 编写插件 (仅最小实例可用) 
- nbrs_py: nbrs Python Plugin (To-do)
- nbrs_matcher_r6s: nbrs Rainbow Six Siege 战绩查询插件

## To-Do List

<details><summary>nonebot_rs</summary>

- [ ] onebot 通讯方式
  - [ ] HTTP (无限期推迟)
  - [ ] 正向 WS (优先考虑)
  - [x] 反向 WS (使用 axum 实现)
- [x] Onebot v11 标准接口实现(使用 serde 实现)
- [ ] Onebot v12 实现 (v12 发布在即！)
- [ ] matcher
  - [x] Built-in matcher
    - [x] echo (基础应答功能)
    - [x] Rcnb (对话功能实现)
  - [x] built-in rules pre_matchers
- [x] config
  - [x] 基本设置
  - [x] bot 设置
  - [x] Plugin 设置
- [x] Message 构建 API 完善
- [x] Plugin
- [x] 文档
- [x] 定时任务
- [x] 声明宏
- [x] logger(tracing-subscriber)

</details>

<details><summary>nbrs_lua</summary>

- [x] 最小实例
  - [ ] More Developer-friendly api for lua

</details>

<details><summary>nbrs_py</summary>

- [ ] 最小实例

</details>

## 特别鸣谢

[OneBot](https://github.com/botuniverse/onebot): 一个聊天机器人应用接口标准，旨在统一不同聊天平台上的机器人应用开发接口，使开发者只需编写一次代码即可应用到多种机器人平台。

[Nonebot2](https://github.com/nonebot/nonebot2): 可扩展的 Python 异步机器人框架。(本项目的思路来源与模仿对象，妈！)

> 开发者只是一个非专业半吊子编程爱好者，如果发现 Bug || 低效算法 || 脱裤子放屁操作，请不吝指教(务必 Issue)

## 说明

简单说明一下项目运行配置

```toml
[global]
host = "127.0.0.1"
port = 8088
debug = true
superusers = ["YourID"]
nicknames = ["nickname"]
command_starts = ["/"]
access_token = "AccessToken"

[bots.BotID]
superusers = ["YourID"]
nicknames = ["nickname"]
command_starts = ["/"]
access_token = "AccessToken"
```

global 设置所有 bot 生效，特别设置后 global 设置将被覆盖。

nbrs 最小实例请看 nonebot_rs/src/bin/minimal.rs 或 nbrs_no4/src/main.rs

matcher 声明请看 nonebot_rs/src/builtin/echo.rs

scheduler 声明可以查看 nbrs_no4/src/clock.rs

目前本项目 Api 初步稳定。
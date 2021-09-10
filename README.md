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

基于 nonebot2 思路 Onebot SDK Rust 实现。

计划是实现 Nonebot2 的完整架构，但是由于 rust 的安全性设计，不可避免的出现了一些魔改，正在努力把开发接口包装成类似的样式(是我太菜了)。

API文档地址：[Docs.rs](https://docs.rs/nonebot_rs/)

## 各项目简介

- nonebot_rs：nbrs 本体
- nbrs_no4：nbrs 实例项目
- nbrs_matcher_r6s：nbrs Rainbow Six Siege 战绩查询插件
- nbrs_py：使用 Nonebot_rs 作为核心的 Python module (未达到可用状态)

## To-Do List

<details><summary>nonebot_rs</summary>

- [ ] onebot 通讯方式
  - [ ] HTTP (无限期推迟)
  - [ ] 正向 WS (优先考虑)
  - [x] 反向 WS (使用 axum 实现)
- [x] Onebot v11 标准接口实现(使用 serde 实现)
  - [x] Event
  - [x] Message
  - [x] Api
- [x] Built-in Handler
  - [x] logger(tracing-subscriber)
  - [x] echo (基础应答功能)
  - [x] Rcnb (对话功能实现，目前写法还很丑陋···想办法打包中)
- [x] built-in rules pre_matchers
- [x] Nbconfig
  - [x] 基本设置
  - [x] bot 设置
  - [x] Matcher 设置
  - [x] 定时任务设置
- [ ] Message 构建 API 完善
- [x] 插件式 Matcher 实现
  - [x] prematcher
  - [x] rules
  - [x] handler
  - [x] aftermatcher
  - [x] Matcher Api
  - [x] 临时 Matcher 实现对话
- [x] 文档
- [x] 定时任务
- [x] 声明宏
- [ ] 使用 pyo3 搭建 nonebot-rs 版 Python 库(又绕回来了.jpg)

</details>

<details><summary>nbrs_py</summary>
Nothing yet.
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

[bots.BotID]
superusers = ["YourID"]
nicknames = ["nickname"]
command_starts = ["/"]
```

global 设置对每个未指定 bot 都有效，~~当在 global 外特别设置一个 bot 后，所有 global 设置对该 bot 全部失效~~ 可以仅指定部分属性。

最小实例请看 nonebot_rs/bin/minimal.rs ，matcher 等等声明请看 builtin 中各项(锐意迭代中)。

目前本项目处于 ~~非常不稳定阶段，项目结构、API 均为待定~~ Api 初步稳定，感兴趣的同学可以 Star 一下以后再来看看(厚颜无耻)
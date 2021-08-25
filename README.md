# Nonebot-rs

基于 rust 的 nonebot2 思路 Onebot 实现

计划是实现 Nonebot2 的完整架构，但是由于 rust 的安全性设计，不可避免的出现了一些魔改，正在努力把开发接口包装成类似的样式（是我太菜了）。

## To-Do List

- [x] 基本的 WebSocket，接收发送（使用 axum 实现）
- [ ] Onebot 标准接口实现（使用 serde 实现）
  - [x] Event
  - [x] Message
  - [ ] Api
- [x] But-in Handler
  - [x] logger(tracing-subscriber)
  - [x] echo
- [ ] Nbconfig
  - [x] 基本设置
  - [x] bot 设置
  - [ ] Matcher 设置
  - [ ] 定时任务设置
- [x] 插件式 Matcher 实现
  - [x] prematcher
  - [x] rules
  - [x] handler
  - [x] aftermatcher
  - [ ] Matcher Api
  - [ ] 临时 Matcher 实现对话
- [ ] 定时任务插件
- [ ] 实现属性宏声明插件，便于二次开发
- [ ] 模块化分离各组件
- [ ] 使用 pyo3 搭建 nonebot-rs 版 Python 库（又绕回来了.jpg）

## 特别鸣谢

[OneBot](https://github.com/botuniverse/onebot): 一个聊天机器人应用接口标准，旨在统一不同聊天平台上的机器人应用开发接口，使开发者只需编写一次代码即可应用到多种机器人平台。

[Nonebot2](https://github.com/nonebot/nonebot2): 可扩展的 Python 异步机器人框架。（本项目的思路来源与模仿对象，妈！）

> 开发者只是一个非专业半吊子编程爱好者，如果发现 Bug || 低效算法 || 脱裤子放屁操作，请不吝指教（务必 Issue）

## 说明

简单说明一下项目运行配置

```toml
[global]
host = "127.0.0.1"
port = 8088
debug = true
superuser = ["YourID"]
nickname = ["nickname"]
command_start = ["/"]

[bots.BotID]
superuser = ["YourID"]
nickname = ["nickname"]
command_start = ["/"]
```

global 设置对每个未指定 bot 都有效，当在 global 外特别设置一个 bot 后，所有 global 设置对该 bot 全部失效（包括未指定项，当然应该也会报错）

最小实例请看 bin/minimal.rs ，matcher 等等声明请看 builtin 中各项（锐意迭代中）。
# Windows Focus Monitor (Rust)

一个基于 Rust 语言开发的轻量级 Windows 窗口焦点监控工具。该程序利用 Windows API 的事件钩子（WinEventHook）实时捕获当前活跃窗口的切换，并输出其标题、进程 ID（PID）以及可执行文件路径。

## 🚀 功能特性

* **实时监控**：通过 `SetWinEventHook` 监听 `EVENT_SYSTEM_FOREGROUND` 事件，响应迅速。
* **详细信息**：输出内容包括窗口标题、所属进程 PID 以及对应的磁盘路径。
* **轻量高效**：直接调用 Windows 底层 API，无需轮询（Polling），极低 CPU 占用。
* **安全性**：使用 Rust 语言开发，在调用 `unsafe` Windows API 的同时保持了外层的安全封装。

## 🛠️ 技术栈

* **语言**: Rust
* **库**: `windows-rs` (Microsoft 官方提供的 Windows API 绑定)
* `Win32::UI::Accessibility`: 用于设置事件钩子。
* `Win32::System::ProcessStatus`: 用于获取进程模块信息。
*  `Win32::UI::WindowsAndMessaging`: 用于消息循环处理。


## 📋 运行预览

当程序运行时，每当你切换当前使用的窗口，终端都会输出如下信息：

```text
--- Focus Switched ---
Title: Visual Studio Code
PID : 12408
Path: C:\Users\Admin\AppData\Local\Programs\Microsoft VS Code\Code.exe
----------------

```

## 📦 快速开始

### 前提条件

1. 安装 [Rust 编译环境](https://www.rust-lang.org/learn/get-started) (Cargo)。
2. 确保操作系统为 Windows 10/11。

### 配置文件 (Cargo.toml)

在你的项目 `Cargo.toml` 中，请确保包含了以下依赖和 features：

```toml
[dependencies]
windows = { version = "0.58", features = [
    "Win32_Foundation",
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_Accessibility",
    "Win32_System_Threading",
    "Win32_System_ProcessStatus",
] }
```

### 构建与运行

1. 克隆或复制本项目代码。
2. 在项目根目录下运行：
```bash
cargo run

```



## 🔍 实现原理

1. **事件钩子**：程序通过 `SetWinEventHook` 注册了一个全局钩子，专门监听系统前台窗口变化事件。
2. **消息循环**：通过 `GetMessageW` 和 `DispatchMessageW` 维持一个标准的消息泵，以确保钩子回调能够被系统正常触发。
3. **进程查询**：当监听到焦点变化时，利用 `GetWindowThreadProcessId` 获取 PID，并配合 `OpenProcess` 和 `K32GetModuleFileNameExW` 溯源该窗口对应的可执行文件路径。

## ⚠️ 注意事项

* **权限问题**：对于以管理员权限运行的窗口（如任务管理器或某些安装程序），如果监控程序本身没有管理员权限，可能无法获取其进程路径。
* **编码**：程序使用 UTF-16 转换，支持多语言（中文、日文等）窗口标题的正确显示。

## 📄 许可证

MIT License

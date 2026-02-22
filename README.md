# Socket.IO Client for WebAssembly

[English](#english) | [中文](#中文)

---

## English

### Overview

`socketio-client-wasm` is a Rust-based Socket.IO client library designed for WebAssembly (WASM) environments. This project provides Rust bindings for the Socket.IO JavaScript client, enabling developers to build real-time web applications using Rust and WebAssembly.

### Features

- 🦀 **Pure Rust API** - Idiomatic Rust interface for Socket.IO
- 🌐 **WebAssembly Support** - Runs in the browser via WASM
- ⚡ **Async/Await** - Full support for asynchronous operations
- 🔌 **Event-Driven** - Component-based event emitter pattern
- 🎯 **Type-Safe** - Leverages Rust's type system for safer code
- 📦 **Modular Architecture** - Organized into multiple crates for better maintainability

### Project Structure

The project is organized as a Rust workspace with the following components:

```
socketio-client-wasm/
├── src/                          # Main library source
│   ├── io.rs                     # Socket.IO factory and protocol
│   ├── lib.rs                    # Library entry point
│   └── prelude.rs                # Convenient re-exports
├── crates/
│   ├── scw-socket-io/                # Core Socket.IO implementation
│   ├── scw-component-emitter/        # Event emitter trait and implementation
│   ├── scw-js-raw/                   # JavaScript interop utilities
│   └── scw-impl-emitter-macro/       # Procedural macros for emitters
└── examples/
    └── hello-world/              # Basic usage example
```

### Core Components

#### 1. **scw-socket-io**
The core Socket.IO client implementation providing:
- Socket connection management
- Event emission and listening
- Acknowledgment support
- Engine.IO integration

#### 2. **scw-component-emitter**
Event emitter pattern implementation with:
- Synchronous and asynchronous event listeners
- Type-safe event handling
- Support for `on`, `once`, and `off` operations

#### 3. **scw-js-raw**
Low-level JavaScript interop utilities:
- JavaScript type wrappers
- Global object access
- Promise and Future integration
- Type conversion helpers

### Quick Start

Here's a simple example of connecting to a Socket.IO server:

```rust
use socketio_client_wasm::prelude::*;
use gloo::console;

fn main() {
    // Create a socket instance
    let socket = io("http://localhost:3000/")
        .auto_connect(false)
        .new();

    let socket2 = socket.clone();

    // Handle connection event
    socket.async_on_connect(async move || {
        console::log!("Connected");

        // Emit event with acknowledgment
        let response = socket2
            .emit_with_ack("some_event")
            .await
            .expect("Emit Failed");
        console::log!("ack response:", response);
    });

    // Handle disconnection event
    socket.on_disconnect(|reason| {
        console::log!(format!("Disconnected: {}", reason));
    });

    // Connect to the server
    socket.connect();
}
```

### Building

To build the project:

```bash
cargo build
```

To build for WebAssembly:

```bash
cargo build --target wasm32-unknown-unknown
```

### Requirements

- Rust 2024 edition or later
- WebAssembly target support
- A Socket.IO server to connect to

### Dependencies
This project includes the Socket.IO JavaScript client library (version 4.6.0),
which is downloaded during build. Socket.IO is licensed under the MIT License.

See the [Socket.IO project](https://github.com/socketio/socket.io) for more information.

### API Overview

#### Creating a Socket

```rust
let socket = io("http://localhost:3000/")
    .auto_connect(false)
    .new();
```

#### Event Listeners

```rust
// Synchronous listener
socket.on_connect(|| {
    println!("Connected!");
});

// Asynchronous listener
socket.async_on_connect(async || {
    // Async operations here
});

// Custom events
socket.on("message", |data| {
    println!("Received: {:?}", data);
});
```

#### Emitting Events

```rust
// Simple emit
socket.emit("event_name");

// Emit with data
socket.emit1("event_name", data);

// Emit with acknowledgment
let response = socket.emit_with_ack("event_name").await?;
```

### Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

### License

This project is licensed under the MIT License.
See [LICENSE](LICENSE) for details.

---

## 中文

### 概述

`socketio-client-wasm` 是一个基于 Rust 的 Socket.IO 客户端库，专为 WebAssembly (WASM) 环境设计。该项目为 Socket.IO JavaScript 客户端提供了 Rust 绑定，使开发者能够使用 Rust 和 WebAssembly 构建实时 Web 应用程序。

### 特性

- 🦀 **纯 Rust API** - 符合 Rust 习惯的 Socket.IO 接口
- 🌐 **WebAssembly 支持** - 通过 WASM 在浏览器中运行
- ⚡ **异步/等待** - 完全支持异步操作
- 🔌 **事件驱动** - 基于组件的事件发射器模式
- 🎯 **类型安全** - 利用 Rust 的类型系统提供更安全的代码
- 📦 **模块化架构** - 组织成多个 crate 以提高可维护性

### 项目结构

该项目组织为一个 Rust 工作空间，包含以下组件：

```
socketio-client-wasm/
├── src/                          # 主库源代码
│   ├── io.rs                     # Socket.IO 工厂和协议
│   ├── lib.rs                    # 库入口点
│   └── prelude.rs                # 便捷的重导出
├── crates/
│   ├── scw-socket-io/                # 核心 Socket.IO 实现
│   ├── scw-component-emitter/        # 事件发射器 trait 和实现
│   ├── scw-js-raw/                   # JavaScript 互操作工具
│   └── scw-impl-emitter-macro/       # 发射器的过程宏
└── examples/
    └── hello-world/              # 基本使用示例
```

### 核心组件

#### 1. **scw-socket-io**
核心 Socket.IO 客户端实现，提供：
- Socket 连接管理
- 事件发射和监听
- 确认支持
- Engine.IO 集成

#### 2. **scw-component-emitter**
事件发射器模式实现，包括：
- 同步和异步事件监听器
- 类型安全的事件处理
- 支持 `on`、`once` 和 `off` 操作

#### 3. **scw-js-raw**
底层 JavaScript 互操作工具：
- JavaScript 类型包装器
- 全局对象访问
- Promise 和 Future 集成
- 类型转换辅助函数

### 快速开始

以下是连接到 Socket.IO 服务器的简单示例：

```rust
use socketio_client_wasm::prelude::*;
use gloo::console;

fn main() {
    // 创建 socket 实例
    let socket = io("http://localhost:3000/")
        .auto_connect(false)
        .new();

    let socket2 = socket.clone();

    // 处理连接事件
    socket.async_on_connect(async move || {
        console::log!("已连接");

        // 发射带确认的事件
        let response = socket2
            .emit_with_ack("some_event")
            .await
            .expect("发射失败");
        console::log!("确认响应:", response);
    });

    // 处理断开连接事件
    socket.on_disconnect(|reason| {
        console::log!(format!("已断开连接: {}", reason));
    });

    // 连接到服务器
    socket.connect();
}
```

### 构建

构建项目：

```bash
cargo build
```

为 WebAssembly 构建：

```bash
cargo build --target wasm32-unknown-unknown
```

### 要求

- Rust 2024 版本或更高
- WebAssembly 目标支持
- 要连接的 Socket.IO 服务器

### 依赖项
本项目包含 Socket.IO JavaScript 客户端库（版本 4.6.0），该库在构建过程中自动下载。Socket.IO 遵循 MIT 许可证授权使用。
更多信息请参阅 [Socket.IO 项目主页](https://github.com/socketio/socket.io)。

### API 概览

#### 创建 Socket

```rust
let socket = io("http://localhost:3000/")
    .auto_connect(false)
    .new();
```

#### 事件监听器

```rust
// 同步监听器
socket.on_connect(|| {
    println!("已连接！");
});

// 异步监听器
socket.async_on_connect(async || {
    // 这里执行异步操作
});

// 自定义事件
socket.on("message", |data| {
    println!("收到: {:?}", data);
});
```

#### 发射事件

```rust
// 简单发射
socket.emit("event_name");

// 带数据发射
socket.emit1("event_name", data);

// 带确认发射
let response = socket.emit_with_ack("event_name").await?;
```

### 贡献

欢迎贡献！请随时提交问题或拉取请求。

### 许可证

本项目遵循MIT许可证授权。
详情请参阅[LICENSE](LICENSE)文件。

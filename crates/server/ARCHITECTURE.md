# Server 架构

这份文档描述 `crates/server` 的模块分层、核心数据流和运行结构。

和 [DESIGN.md](./DESIGN.md) 的分工：

- `DESIGN.md` 重点描述对外协议、职责边界、运行约束
- `ARCHITECTURE.md` 重点描述代码组织、模块依赖、请求流转和启动路径

---

## 总览

`server` 当前可以拆成 5 层：

1. 入口层
2. 运行时与配置层
3. HTTP 服务层
4. 输入执行层
5. 辅助能力层

```mermaid
flowchart TD
    A[main.rs] --> B[lib.rs]
    B --> C[runtime.rs]
    B --> D[server.rs]
    B --> E[export.rs]

    D --> D1[server/router.rs]
    D --> D2[server/handlers.rs]
    D --> D3[server/auth.rs]
    D --> D4[server/error.rs]
    D --> D5[server/startup.rs]

    D2 --> F[protocol.rs]
    D2 --> G[input.rs]

    G --> G1[input/executor.rs]
    G --> G2[input/clipboard.rs]
    G --> G3[input/keyboard.rs]
    G --> G4[input/keymap.rs]
    G --> G5[input/platform.rs]
    G5 --> G6[input/windows.rs]
    G5 --> G7[input/macos.rs]

    D5 --> H[network.rs]
    D5 --> I[import_config.rs]
    D5 --> J[qr.rs]
    E --> E1[export/typescript.rs]
```

---

## 目录职责

### 入口层

- `main.rs`
  - 初始化日志
  - 解析运行命令
  - 调用 `run` 或 `export_typescript_bindings`
- `lib.rs`
  - 暴露 crate 公共入口
  - 聚合内部模块

### 运行时与配置层

- `cli.rs`
  - 定义 clap CLI 结构
- `runtime.rs`
  - 加载和合并配置
  - 产出 `RuntimeCommand` 和 `RuntimeOptions`

### HTTP 服务层

- `server.rs`
  - 服务器生命周期入口
  - 监听 TCP
  - 优雅退出
- `server/router.rs`
  - 路由装配
  - CORS 和 body limit
  - app state 挂载
- `server/handlers.rs`
  - `/health`
  - `/api/auth-check`
  - `/api/capabilities`
  - `/api/action`
- `server/auth.rs`
  - Bearer token 校验
- `server/error.rs`
  - HTTP 错误到状态码映射
- `server/startup.rs`
  - 启动日志
  - 局域网访问地址输出
  - 导入 URL / 二维码输出

### 输入执行层

- `protocol.rs`
  - `ServerAction`
  - `KeyChord`
  - 请求响应模型
  - 协议层校验
- `input.rs`
  - 输入模块门面
  - 对外暴露 `execute_action`
  - 定义 `InputError`
- `input/executor.rs`
  - `ServerAction -> input-text / key-sequence` 分发
- `input/clipboard.rs`
  - `input-text` 的剪贴板写入与粘贴触发
- `input/keyboard.rs`
  - chord / sequence 编排
  - modifier 排序
- `input/keymap.rs`
  - `keyboard_types::Code -> 平台键描述`
- `input/platform.rs`
  - 选择当前操作系统后端
  - 提供平台原生粘贴快捷键和 capability 过滤
- `input/windows.rs`
  - Win32 `SendInput`
  - Windows 平台输入注入
- `input/macos.rs`
  - CoreGraphics `CGEvent`
  - macOS 按键映射和辅助功能权限检查

### 辅助能力层

- `export.rs`
  - 导出 TypeScript 类型文件
- `export/typescript.rs`
  - 基于 `specta` / `specta-serde` 生成共享 TypeScript 类型
- `network.rs`
  - 本机地址探测和访问 URL 推荐
- `import_config.rs`
  - 导入 URL 构造
- `qr.rs`
  - 终端二维码渲染

---

## 启动路径

### `serve`

```mermaid
sequenceDiagram
    participant Main as main.rs
    participant Runtime as runtime.rs
    participant Server as server.rs
    participant Router as server/router.rs
    participant Startup as server/startup.rs

    Main->>Runtime: parse_runtime_command()
    Runtime-->>Main: RuntimeCommand::RunServer(options)
    Main->>Server: run(options)
    Server->>Router: build_router(auth_token)
    Server->>Server: bind TCP listener
    Server->>Startup: log_startup_guide(options)
    Startup->>Startup: 输出 health/auth/capabilities/action URL
    Startup->>Startup: 探测 LAN 地址
    Startup->>Startup: 生成导入 URL 和二维码
    Server->>Server: axum::serve(...)
```

### `export-types`

```mermaid
sequenceDiagram
    participant Main as main.rs
    participant Runtime as runtime.rs
    participant Export as export.rs
    participant Generator as export/typescript.rs

    Main->>Runtime: parse_runtime_command()
    Runtime-->>Main: RuntimeCommand::ExportTypes
    Main->>Export: export_typescript_bindings(output_path)
    Export->>Generator: render_typescript_bindings()
    Generator->>Generator: specta register protocol types
    Generator->>Generator: specta_serde apply serde wire format
    Generator->>Generator: specta_typescript export bindings
    Export->>Export: 写入 packages/app/src/types/server.ts
```

---

## 请求流转

### `/api/action`

```mermaid
flowchart TD
    A[HTTP POST /api/action] --> B[server/router.rs]
    B --> C[server/handlers.rs::execute_action]
    C --> D[server/auth.rs]
    D --> E[protocol.rs::ServerAction.validate]
    E --> F[input::execute_action]
    F --> G[input/executor.rs]
    G --> H1[input/clipboard.rs]
    G --> H2[input/keyboard.rs]
    H2 --> I[input/keymap.rs]
    H2 --> J[input/windows.rs]
    H1 --> H2
    C --> K[server/error.rs]
```

处理顺序：

1. router 接收请求并完成基础 middleware 处理
2. handler 校验 Bearer token
3. handler 反序列化并校验动作结构
4. 输入层执行 `input-text` 或 `key-sequence`
5. 执行错误被映射成 HTTP 响应

### `/api/auth-check`

```mermaid
flowchart LR
    A[HTTP GET /api/auth-check] --> B[router]
    B --> C[handlers::auth_check]
    C --> D[auth::authorize]
    D --> E[返回 ok:true]
```

### `/api/capabilities`

```mermaid
flowchart LR
    A[HTTP GET /api/capabilities] --> B[router]
    B --> C[handlers::capabilities]
    C --> D[auth::authorize]
    D --> E[input::supported_codes]
    E --> F[返回 supported_codes]
```

### `/health`

```mermaid
flowchart LR
    A[HTTP GET /health] --> B[router]
    B --> C[handlers::health]
    C --> D[返回纯文本 ok]
```

---

## 输入层内部结构

`input` 子系统内部也分成 3 层：

1. 动作分发
2. 平台无关编排
3. 平台相关落地

```mermaid
flowchart TD
    A[input::execute_action] --> B[input/executor.rs]
    B --> C[input/clipboard.rs]
    B --> D[input/keyboard.rs]
    D --> E[input/keymap.rs]
    D --> F[input/platform.rs]
    F --> G[input/windows.rs]
    F --> H[input/macos.rs]
```

### `input-text`

```mermaid
flowchart LR
    A[input-text] --> B[clipboard.rs 写剪贴板]
    B --> C[短暂等待]
    C --> D[keyboard.rs 发送 Ctrl+V chord]
    D --> E[windows.rs -> SendInput]
```

### `key-sequence`

```mermaid
flowchart LR
    A[key-sequence] --> B[keyboard.rs]
    B --> C[遍历 sequence]
    C --> D[对每个 KeyChord 排序 modifier]
    D --> E[keymap.rs 校验 Code]
    E --> F[platform.rs 选择后端]
    F --> G[windows.rs -> SendInput]
    F --> H[macos.rs -> CGEvent]
```

---

## 错误边界

当前错误大致分为 3 类：

- 协议校验错误
  - 由 `protocol.rs` 产生
  - 例如空文本、空序列、空 chord
- 认证错误
  - 由 `server/auth.rs` 产生
  - 例如 token 缺失或错误
- 执行错误
  - 由 `input` 子系统产生
  - 例如剪贴板失败、平台未实现、键码未支持、输入注入失败

HTTP 映射统一收敛在：

- `server/error.rs`

这样做的目的是：

- handler 不直接编码状态码策略
- 输入层只关心执行语义，不关心 HTTP
- 协议层只关心数据是否合法

---

## 当前架构特征

当前这套组织的特点是：

- 对外协议和内部执行边界已经分离
- HTTP 层和输入层已经分层
- Windows/macOS 平台细节已分别收敛到 `input/windows.rs` 和 `input/macos.rs`
- 类型导出、导入配置、二维码等辅助能力不再和请求处理代码混在一起
- 共享 TypeScript 类型由 Rust 协议类型生成，而不是单独手写维护

当前仍然保留的一个现实选择：

- `keyboard_types::Code` 到 Windows 事件的映射现在仍是平台映射表驱动
- `input/keymap.rs` 同时承担“支持的键集合”和“Windows 虚拟键映射表”的单一真相来源
- `ServerCode` 在共享 TypeScript 中仍被导出为宽泛字符串，真实支持子集通过 `/api/capabilities` 暴露
- 如果后续要进一步严格贴近“物理键位”语义，可以继续把 `input/keymap.rs` 和 `input/windows.rs` 演进到 scan code 路线

---

## 后续演进点

如果继续扩展，最自然的方向是：

- 为 `input` 增加更多平台实现
- 为导出链路引入更严格的生成机制
- 在 `protocol.rs` 中为 `KeyChord` 增加更多元数据
  - 例如 `hold_ms`
  - 例如 `repeat`
- 在 HTTP 层增加更稳定的错误响应结构

当前结构已经为这些方向预留了模块边界，不需要再次整体打散。

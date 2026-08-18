# Server 设计

这份文档描述 `crates/server` 采用的接口设计、职责边界和运行约束。

本文优先描述对外协议和目标结构；实现应向这里收敛。

架构分层、模块关系和运行链路见 [ARCHITECTURE.md](./ARCHITECTURE.md)。

## 定位

`server` 是运行在电脑上的本地控制服务，负责把来自前端或客户端的动作请求落到当前桌面焦点位置。

当前职责：

- 提供本地 HTTP API
- 校验 Bearer token
- 执行动作请求
- 在启动时输出本机访问地址
- 生成移动端导入配置 URL 与二维码
- 导出共享 TypeScript 类型

明确不负责：

- 前端托管
- 客户端 UI
- 云端中继
- 账号体系
- 多设备授权列表

说明：

- `server` 现在不会做“设备发现协议”或“配对流程”，但会在本地启动时探测可用局域网地址，并把它用于导入配置展示
- 导入配置是启动辅助能力，不是单独暴露一组导入 API

---

## 运行命令

CLI 当前有两个子命令：

```text
vibe-coding-remote serve
vibe-coding-remote export-types
```

### `serve`

启动本地 API 服务。

支持参数：

- `--config <path>`
- `--host <ip>`
- `--port <u16>`
- `--auth-token <token>`

### `export-types`

导出 TypeScript 类型定义，供前端和共享包复用。

当前导出实现：

- 基于 Rust 协议类型生成
- 通过 `specta` 导出 TypeScript
- 通过 `specta-serde` 对齐 `serde` 序列化语义
- 当前默认输出到共享包内的单个 `server.ts`

支持参数：

- `--output <path>`

默认输出路径：

```text
packages/app/src/types/server.ts
```

---

## API 概览

当前对外暴露 4 个 HTTP 接口：

```http
POST /api/action
GET /api/auth-check
GET /api/capabilities
GET /health
```

附加运行时约束：

- `/api/action`、`/api/auth-check` 和 `/api/capabilities` 需要 Bearer token
- `/health` 不需要认证
- 已启用 permissive CORS，允许跨源前端访问
- 请求体大小限制为 `64 KiB`

`/health` 成功时返回：

```text
ok
```

`/api/auth-check` 成功时返回：

```json
{
  "ok": true
}
```

`/api/capabilities` 成功时返回：

```json
{
  "supported_codes": ["Enter", "Tab", "ControlLeft"]
}
```

`/api/action` 成功时返回：

```json
{
  "ok": true
}
```

---

## 动作模型

动作接口统一为 2 类：

1. `input-text`
2. `key-sequence`

Rust 内部模型目标为：

```rust
use keyboard_types::Code;

struct KeyChord {
    keys: Vec<Code>,
}

enum ServerAction {
    InputText { text: String },
    KeySequence { sequence: Vec<KeyChord> },
}
```

设计说明：

- `input-text`
  - 用于输入任意非空文本
  - 当前实现策略是剪贴板写入后再触发粘贴
- `key-sequence`
  - 用于发送任意按键、组合键和按键序列
  - 按键名直接采用 `keyboard-types::Code`
  - `Code` 表示物理键位，不表示字符值
  - 导出的共享 TypeScript 中，`ServerCode` 当前被表示为 `string`
  - 每一步按键组合由 `KeyChord` 显式表示，而不是直接使用裸数组
  - server 只实现当前平台支持的 `Code` 子集，可通过 `/api/capabilities` 获取

`sequence` 的语义：

- 外层数组按顺序执行
- 每个 `KeyChord` 表示一组同时按下的按键
- 每一组按键会完整执行按下和释放，再进入下一组

语义示例：

- `[{ keys: [Enter] }]` 表示发送一次 Enter
- `[{ keys: [ControlLeft, KeyV] }]` 表示发送一次 `Ctrl+V`
- `[{ keys: [ControlLeft, KeyV] }, { keys: [Enter] }]` 表示先粘贴，再按 Enter

请求体示例：

输入文本：

```json
{
  "action": {
    "type": "input-text",
    "text": "你好，世界"
  }
}
```

发送一次 Enter：

```json
{
  "action": {
    "type": "key-sequence",
    "sequence": [
      {
        "keys": ["Enter"]
      }
    ]
  }
}
```

发送 `Ctrl+V`：

```json
{
  "action": {
    "type": "key-sequence",
    "sequence": [
      {
        "keys": ["ControlLeft", "KeyV"]
      }
    ]
  }
}
```

先粘贴，再按 Enter：

```json
{
  "action": {
    "type": "key-sequence",
    "sequence": [
      {
        "keys": ["ControlLeft", "KeyV"]
      },
      {
        "keys": ["Enter"]
      }
    ]
  }
}
```

---

## 认证

受保护接口统一使用 Bearer token。

请求头：

```http
Authorization: Bearer <token>
```

规则：

- `/api/action` 必须认证
- `/api/auth-check` 必须认证
- `/api/capabilities` 必须认证
- `/health` 不认证

认证失败当前返回：

```http
401 Unauthorized
```

响应文本：

```text
missing or invalid authorization token
```

---

## 文本输入实现

当前文本输入主要走：

```text
剪贴板写入 + Ctrl+V
```

执行流程：

1. 收到 `input-text`
2. 将文本写入系统剪贴板
3. 等待一个很短的稳定时间片
4. 模拟一次 `Ctrl+V`

这样做的原因：

- 不依赖当前键盘布局逐字生成字符
- 对中文、语音输入结果和长文本更稳
- 比逐字符键盘模拟更适合当前使用场景

`key-sequence` 则通过桌面输入注入实现。

Windows 上的目标实现方式：

- 协议层直接使用 `keyboard-types::Code`
- server 在本地把 `Code` 映射到 Windows 输入事件
- 一个 chord 内先按下所有键，再按相反顺序释放
- 一个 sequence 中的各个 chord 顺序执行

共享类型导出约束：

- `ServerAction`、`KeyChord`、`ApiResponse` 等协议结构从 Rust 类型生成
- `ServerCode` 不在静态类型里展开为完整键名联合类型
- 前端应通过 `/api/capabilities` 获取当前 server 实际支持的键集合

当前平台现状：

- Windows 使用 Win32 `SendInput`；
- macOS 使用 CoreGraphics `CGEvent`，并要求“辅助功能”权限；
- 文本输入在 Windows 使用 `Ctrl+V`，在 macOS 使用 `Command+V`；
- `/api/capabilities` 只返回当前平台已映射的按键；
- 其他平台可编译通用服务代码，但桌面输入动作会返回未支持错误。

---

## 导入配置

`server` 启动时会输出访问指引，并尝试生成可供移动端扫描导入的配置。

这部分当前包含：

- 探测推荐局域网访问地址
- 构造导入 URL
- 打印二维码到终端日志

导入 URL 当前使用自定义 scheme：

```text
vibecodingremote://import
```

当前导入参数：

- `v=1`
- `endpoint=<server_base_url>`
- `token=<auth_token>`

对应载荷模型：

```rust
struct ImportPayload {
    v: u8,
    endpoint: String,
    token: String,
}
```

边界说明：

- 如果服务绑定在 `127.0.0.1`，不会生成可供手机使用的局域网导入地址
- 如果找不到合适的私网 IPv4 地址，也不会生成推荐导入地址
- 导入 URL 包含明文 token，属于敏感信息

---

## 配置

配置来源优先级：

```text
CLI > ENV > Config File > Default
```

当前配置项：

- `host`
- `port`
- `auth_token`

默认值：

- `host = 127.0.0.1`
- `port = 8765`
- `auth_token` 无默认值

配置文件：

- 默认读取 `config.toml`
- 可通过 `--config` 指定其他路径

环境变量前缀：

```text
VIBE_CODING_REMOTE_
```

示例：

- `VIBE_CODING_REMOTE_HOST`
- `VIBE_CODING_REMOTE_PORT`
- `VIBE_CODING_REMOTE_AUTH_TOKEN`

启动约束：

- 如果最终没有得到非空 `auth_token`，服务拒绝启动

---

## 错误语义

目标错误语义：

- `400 Bad Request`
  - JSON 语法错误
  - `input-text.text` 为空
  - `key-sequence.sequence` 为空
  - `key-sequence.sequence[].keys` 为空
  - `key-sequence` 使用了当前平台尚未支持的 `keyboard-types::Code`
  - 请求结构不合法
- `401 Unauthorized`
  - 缺少 token
  - Bearer scheme 不合法
  - token 错误
- `413 Payload Too Large`
  - 请求体超过 `64 KiB`
- `415 Unsupported Media Type`
  - `POST /api/action` 未使用 `Content-Type: application/json`
- `422 Unprocessable Entity`
  - JSON 可解析，但字段值无法映射到当前类型
  - 例如不存在的 `keyboard-types::Code` 名称
- `500 Internal Server Error`
  - 剪贴板失败
  - 桌面输入注入失败
  - 当前平台未实现输入注入

补充说明：

- 具体状态码仍可能受 framework extractor 行为影响
- 如果后续需要严格稳定的外部契约，可以再把框架级错误统一收口

---

## 非目标

当前不做：

- 前端页面托管
- 独立客户端交互设计
- 云端转发或云端中继
- 账号系统
- 远程设备管理后台
- 多设备授权列表管理

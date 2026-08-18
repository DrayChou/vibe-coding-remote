# Vibe Coding 遥控器 / Vibe Coding Remote

Vibe Coding 遥控器是一个把手机输入体验桥接到电脑当前焦点位置的小工具。

它的核心思路很直接：不再试图重新发明一套电脑端语音输入，而是复用手机上已经很好用的输入法，尤其是语音输入、联想输入和中文输入生态。

## 为什么做这个项目

这个项目来自一个很具体的痛点。

最近我大量在用 AI 编程工具，例如 Codex、Claude Code 一类产品。用得越多就越明显地感觉到，很多时候表达速度的瓶颈不是想法本身，而是打字。于是我很自然地开始尝试用语音输入代替一部分键盘输入。

但电脑上的语音输入体验一直没有让我满意。豆包电脑版、微信电脑版，还有一些接大模型 API 的第三方工具，我都试过。它们各有优点，但整体上还是有几个共通问题：

- 识别效果不如手机上的输入法稳定
- 一些常用词、技术词、口语语气词处理得不够自然
- 把识别结果真正插入到当前光标位置这件事，成功率并不总是可靠
- 在安静环境里，对着电脑说话也不一定是最舒服的方式

反过来，手机上的输入法，尤其是我常用的手机版豆包输入法，体验明显更好。它的语音输入更顺，低音量下识别也更稳，输入法本身的交互已经被打磨得很成熟。所以这个项目真正想做的是：

> 把手机上成熟、顺手、自然的输入体验，直接桥接到电脑当前的工作流里。

它不只适合 AI 编程。只要你的场景是“在手机上说话或输入，再把结果落到电脑上”，它都成立。比如：

- 电脑没有麦克风
- 电脑麦克风收音很差
- 你所在的环境更适合对着手机低声说话
- 你更喜欢手机输入法的语音识别效果和交互手感

## 体验目标

Vibe Coding 遥控器从一开始追求的就不只是“能用”，而是“顺手”。

目前这套交互重点在于：

- 尽量不打断手机输入法，尤其是连续语音输入
- 让高频操作尽量留在手机键盘附近完成
- 把发送、换行、退格这些动作做成低摩擦的闭环
- 让网页更像输入工具，而不是一个普通表单页

一个典型 AI 编程场景会是这样：

1. 手机打开Vibe Coding 遥控器页面，键盘弹出。
2. 按住空格开始语音输入。
3. 等输入法完成最终整理。
4. 点击发送，把文本插到电脑当前光标处。
5. 再触发一次电脑上的 Enter，把这段内容真正发给 AI。

整个过程里，手机承担“输入体验”，电脑承担“落点执行”。

## 目前能做什么

- 手机网页输入文本并发送到电脑当前焦点位置
- 复用手机语音输入法，把整理后的文本直接落到电脑
- 发送任意按键、组合键和按键序列到电脑当前焦点位置
- 启动时打印推荐局域网访问地址
- 启动时生成移动端导入配置二维码，方便手机导入 server 地址和 token
- 前端支持：
  - 深色 / 浅色 / 跟随系统
  - 回车发送 / 回车换行
  - Dock 按钮显隐
  - 最近发送历史
  - 草稿自动保存与恢复

## 在线使用

如果你只是想直接使用前端页面，可以先打开：

```text
https://enximi.github.io/vibe-coding-remote/
```

推荐使用流程：

1. 在电脑上下载并运行本地 `server`
2. 在手机上打开上面的 GitHub Pages 页面
3. 把本地 `server` 的地址和 token 配进页面，或者直接扫码导入
4. 先在电脑上把光标放到目标输入框，再开始在手机上输入或语音输入

这条在线地址的意义是：

- 你不需要自己部署前端
- 用户不需要运行 Vite 或下载整个仓库
- 大家只需要拿到前端页面，再单独运行本地 `server` 即可

需要注意：

- 正式 Desktop App 会把 Mobile Web 资源一起打包，并由同一个局域网 HTTP Server 提供页面和 API；手机直接打开管理页显示的局域网地址即可，避免 HTTPS 页面访问本地 HTTP API 的 mixed-content 问题。
- GitHub Pages 仍可用于前端预览；如果 Pages 连接纯 HTTP 本地 Server，浏览器可能拦截跨协议请求，此时应改用 Desktop App 提供的本地手机页面。

## 平台现状

当前桌面输入主链支持 Windows 和 macOS：

- Windows：系统剪贴板 + `Ctrl+V`，按键通过 Win32 `SendInput` 注入；
- macOS：系统剪贴板 + `Command+V`，按键通过 CoreGraphics `CGEvent` 注入；
- macOS 首次执行桌面动作时需要授予运行该服务的应用“辅助功能”权限；
- Linux 仍可编译通用服务代码，但桌面输入注入尚未实现。

当前保证的是“文本和已声明基础按键进入当前焦点位置”。前台应用识别、窗口/鼠标控制和 Agent 原生状态协议仍属于后续能力，不能仅凭输入主链可用就视为已经支持。

## 仓库结构

```text
vibe-coding-remote/
├─ apps/
│  ├─ web/                  # 手机浏览器 / PWA 控制端
│  └─ desktop/              # Windows / macOS Tauri 管理端
├─ packages/
│  ├─ app/                  # 共享 React UI、状态模型、组件、样式
├─ crates/
│  └─ server/               # Rust 本地服务：API、输入执行、局域网访问地址输出
└─ ...
```

这套结构的原则是：

- `packages/app` 只关心手机输入与遥控交互；
- `apps/web` 提供手机浏览器 / PWA 壳；
- `apps/desktop` 提供 Windows / macOS 管理窗口、托盘和系统权限入口；
- `crates/server` 负责本地 API 与桌面输入执行，并由 Desktop App 同进程启动。

## 快速开始

### 1. 安装依赖

```powershell
cd D:\projects\vibe-coding-remote
pnpm install
```

### 2. Web 开发模式

```powershell
pnpm run dev:web
```

另开一个终端，再启动本地 Rust 服务：

```powershell
just dev-server
```

如果要让手机从局域网访问本机 server，可以用：

```powershell
just dev-server-lan
```

开发时手机访问：

```text
https://你的电脑局域网IP:5173
```

这时：

- 页面资源来自 Vite
- `/api` 由 Vite 代理到 Rust `8765`
- 如果本地不存在 `.cert/dev-cert.pem` 和 `.cert/dev-key.pem`，Vite 会回退为普通 HTTP 开发模式

### 3. 启动桌面管理端

```bash
pnpm run dev:desktop
```

桌面管理端使用 Tauri 2，同一套代码支持 Windows 和 macOS。构建时会同时打包 `apps/web/dist`，桌面 Server 的根地址直接提供手机页面和 API。它负责：

- 同进程启动和重启 Rust Server；
- 展示服务、局域网、端口和输入权限状态；
- 保存监听地址与端口；
- 复制手机连接地址和包含 Token 的完整配置；
- Windows 系统托盘 / macOS 菜单栏；
- 关闭窗口后继续后台运行，并可从托盘重新打开。

正式构建：

```bash
pnpm run build:desktop
```

macOS 产物位于 `target/release/bundle/macos/` 和 `target/release/bundle/dmg/`；Windows 构建会生成对应的安装产物。仍可使用 `just build-server` 单独构建无头 Rust Server。

## 运行模式

### 开发模式

```text
手机浏览器 -> Vite (5173) -> /api -> Rust (8765)
```

### 发布模式

```text
手机浏览器 / App -> 独立部署的前端
                    -> /api -> Rust (8765)
```

发布模式下：

- `server` 只提供 API
- `web` 需要独立部署
- `server` 已开启 CORS，允许跨源前端直接访问

### GitHub Pages 部署

仓库已经包含 GitHub Pages workflow：

- [.github/workflows/deploy-pages.yml](./.github/workflows/deploy-pages.yml)

默认行为是：

1. push 到 `main`
2. GitHub Actions 自动构建 `apps/web`
3. 产物部署到 GitHub Pages

第一次使用时，还需要在 GitHub 仓库设置里把 Pages 的构建来源切到：

```text
Settings -> Pages -> Source -> GitHub Actions
```

如果仓库地址是：

```text
https://github.com/enximi/vibe-coding-remote
```

那么默认 Pages 地址会是：

```text
https://enximi.github.io/vibe-coding-remote/
```

当前 `vite.config.ts` 会在 GitHub Actions 中自动把 `base` 设置为仓库名路径，因此项目页模式可以直接工作。

如果以后改成自定义域名，可以在构建时传入：

```text
VITE_BASE_PATH=/
```

需要注意：

- GitHub Pages 是 HTTPS
- 浏览器通常不允许 HTTPS 页面直接请求 HTTP 的本地 API
- 所以如果前端部署在 Pages，上游 `server` 最好也提供 HTTPS，或者通过 HTTPS 隧道暴露给手机浏览器

## API

`server` 当前通过统一动作接口接收请求：

```http
POST /api/action
```

受保护接口需要 Bearer token：

```http
Authorization: Bearer <token>
```

### 输入文本

把一段文本输入到当前焦点位置。

动作类型为 `input-text`：

```json
{
  "action": {
    "type": "input-text",
    "text": "你好，世界"
  }
}
```

### 发送按键序列

发送任意按键、组合键或连续按键序列。

按键名使用 `keyboard-types` 的 `Code` 命名，表示物理键位。

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

`sequence` 的语义是：

- 外层数组按顺序执行
- 每个对象表示同一组同时按下的按键
- `keys` 使用 `keyboard-types` 的 `Code` 命名
- 每一组按键会完整执行按下和释放，再进入下一组

### `GET /health`

健康检查。

返回：

```text
ok
```

## 文字是怎么进入电脑的

文字输入目前主要走：

> 剪贴板写入 + 粘贴

流程是：

1. 前端调用 `/api/action`
2. Rust 收到 `input-text` 动作
3. Rust 把文本写入系统剪贴板
4. 等待一个很短的时间片
5. Rust 模拟一次 `Ctrl+V`
6. 文本出现在电脑当前焦点位置

这样通常比逐字模拟键盘输入更稳，尤其适合中文、语音输入和长文本。

按键序列则通过 Win32 `SendInput` 模拟。协议层使用 `keyboard-types` 的 `Code` 表示物理键位，server 在 Windows 上把这些键位映射到实际输入事件。

## 常见问题

### 为什么开发时改了 React 文件，`apps/web/dist` 没变化？

因为 Vite 开发服务器的资源在内存里。

开发时要看最新页面，请直接访问 `5173`。`apps/web/dist` 只有执行构建时才会更新。

### Web 独立部署后，为什么页面不能直接发送到电脑？

因为独立部署后的 Web 页面和本地 `server` 不再同源，前端必须明确知道目标 `server` 的 API 地址。

当前最直接的方式是给页面带上 `endpoint` 参数，例如：

```text
https://你的前端域名/?endpoint=http://192.168.1.23:8765/api/action
```

### 为什么手机打不开 `http://你的IP:5173`？

常见原因：

- 手机和电脑不在同一个局域网
- Windows 防火墙没有放行 Node / Vite 的入站连接
- 路由器或校园网启用了客户端隔离
- 当前电脑 IP 已经变化

### 为什么不直接把文字灌进任意桌面输入框？

因为普通桌面应用并没有一个统一、稳定、跨应用的“直接写入文本框”系统 API。

更通用的方案通常是：

- 模拟键盘输入
- 或者通过剪贴板 + 粘贴

Vibe Coding 遥控器目前主要选择后者。

## 路线

后续可以继续扩展的方向包括：

- 补齐移动端客户端体验
- 多端共享更多平台桥接能力
- 设备发现与配对
- WebSocket 实时状态同步
- Linux 桌面输入支持
- 前台应用 Profile 与遥控器交互模型，见 [`docs/REMOTE_CONTROL_REUSE.md`](docs/REMOTE_CONTROL_REUSE.md)

## License

本项目采用 **MIT / Apache-2.0 双授权**。

你可以任选其一使用：

- [LICENSE-MIT](./LICENSE-MIT)
- [LICENSE-APACHE](./LICENSE-APACHE)

除非你明确声明否则，提交到本仓库的贡献也将按相同的双授权方式发布。

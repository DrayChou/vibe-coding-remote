# 遥控器能力复用评估

## 目标

评估 `mi_remote_control`、`remote-mic-app` 与 `open-voice-bridge` 中已经验证过的遥控器交互，哪些适合复用到 Vibe Coding Remote 的“手机浏览器 → 桌面控制”主链。

本阶段只评估并确定接入边界，不接入 RC003 蓝牙、HID 或 ATVV 语音链路。

## 结论

采用 **reuse/extend**，不替换当前 Web + Rust 主链：

- 保留手机浏览器作为主要遥控器；
- 保留 Rust server 作为 Windows/macOS 系统动作执行端；
- 复用遥控器项目的交互模型和状态机设计；
- 不直接搬运 macOS Swift 平台代码；
- 不复制 GPL 项目的实现代码到当前 MIT/Apache-2.0 仓库；
- 将来若支持 RC003，把它实现成新的输入适配器，而不是第二套动作执行主链。

## 现在已经具备的对应能力

当前 Web 遥控面板已经覆盖一部分实体遥控器基础操作：

- Enter、Escape、Tab、Shift+Tab；
- Ctrl+C、Ctrl+V；
- Backspace；
- 上下左右；
- 文本输入与发送；
- 长按连续触发；
- 用户自定义面板布局；
- 手机震动反馈。

这些动作最终都进入统一的 `ServerAction`，因此手机按钮、未来的实体遥控器、Agent Connector 可以共享同一执行入口。

## 推荐复用的能力

### 1. 语义动作层

当前前端仍会发送 `Ctrl+V`、`Ctrl+C` 等具体组合键。跨平台扩展后，建议增加平台无关动作：

```text
submit
cancel
interrupt
copy
paste
undo
redo
next-item
previous-item
```

由桌面后端转换：

```text
copy: Windows Ctrl+C / macOS Command+C
paste: Windows Ctrl+V / macOS Command+V
interrupt: 终端默认 Ctrl+C
```

这是优先级最高、最适合直接复用的遥控器设计。

### 2. Trigger 状态机

可复用 `mi_remote_control` 的交互概念：

- tap；
- hold；
- double tap；
- repeat；
- 全局逃生动作。

Web 端目前已有长按连续触发，但还没有统一的 Trigger 协议。后续应让前端只产生触发事件，映射规则由共享配置决定，避免每个按钮独立编码行为。

### 3. Profile 与控制模式

可复用：

```text
global profile
+ foreground-app profile
+ temporary control layer
+ agent/session context
```

示例：

- Windows Terminal / Ghostty：方向、Enter、Escape、Ctrl+C；
- 浏览器：标签页和页面导航；
- VS Code：面板和编辑器导航；
- Pi / Claude / Codex：批准、拒绝、中断、切换会话。

第一步只需要前台应用识别和 Profile 匹配，不需要先读取桌面输入框内容。

### 4. 安全逃生与失败恢复

应复用以下产品规则：

- 长按固定按钮清空临时模式；
- 网络断开时停止 repeat/hold；
- 页面失焦、触摸取消或 WebSocket 断开时释放所有按下状态；
- 高风险 Agent 动作不能绑定无提示单击；
- 服务端校验动作，不信任前端自报状态。

### 5. Agent 控制层

可复用遥控器项目的 Agent 操作思路，但需要真实 Connector 才能声明支持：

- approve once；
- reject；
- cancel；
- next/previous session；
- show details；
- waiting/working/completed/failed 状态。

在 Connector 完成前，只能称为“终端按键控制”，不能称为 Agent 原生控制。

### 6. 鼠标与窗口动作

如果“me mode”指的是 MiRemote 的 mouse mode，可将以下能力作为后续阶段：

- 鼠标相对移动；
- 左键/右键；
- 滚轮；
- 窗口切换；
- App 轮盘；
- Mission Control / Task View。

这些能力应新增为语义动作并由平台后端实现，不应让 Web 端直接发送平台专用脚本。

## 不建议直接复用的部分

### macOS Swift 系统实现

`mi_remote_control` 的 `IOHIDManager`、`hidutil`、`CGEventTap`、Accessibility 和 SwiftUI 代码不能直接用于 Windows，也不适合作为当前 Rust server 的第二主链。

可以移植行为和测试场景，但平台执行应继续落在 Rust 的 Windows/macOS backend。

### RC003 语音链路

Vibe Coding Remote 的核心优势是复用手机输入法，当前没有必要同时引入：

- BLE GATT；
- ATVV；
- ADPCM；
- BlackHole/VB-CABLE；
- 虚拟麦克风驱动。

这些会扩大范围，却不解决当前“手机输入 → 双平台桌面”的核心问题。

### GPL 源码

`remote-mic-app` 和相关 GPL 实现可用于行为调研和协议理解，但当前仓库采用 MIT/Apache-2.0 双授权，不能直接复制 GPL 代码后继续按现有许可证发布。

`mi_remote_control` 为 MIT，可在保留许可证和来源信息的前提下移植局部算法；但其 Swift/macOS 耦合较深，优先重写小型状态机比逐行翻译更清晰。

## 推荐接入顺序

1. 完成 Windows/macOS 文本与基础按键主链；
2. 增加 `StandardAction` 语义动作；
3. 增加触发状态机与断线释放；
4. 增加前台 App 识别和 Profile；
5. 增加窗口、鼠标等桌面动作；
6. 增加 Pi/Claude/Codex Connector；
7. 只有出现明确需求时，再把 RC003 作为可选输入适配器接到同一个动作总线。

## 最终目标结构

```text
手机 Web / 未来 RC003 / 其他控制器
                  ↓
             Trigger Event
                  ↓
       Mapping + Profile + Context
                  ↓
       StandardAction / AgentCommand
                  ↓
       Windows backend / macOS backend
                  ↓
        当前应用或指定 Agent session
```

这能保证遥控器能力被复用，同时不破坏当前已经工作的 Web 输入主链。

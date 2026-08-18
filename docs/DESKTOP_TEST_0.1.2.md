# Vibe Coding Remote Desktop 0.1.2 联合测试

## 测试产物

### macOS Apple Silicon

```text
dist/VibeCodingRemote-0.1.2-macos-aarch64.dmg
SHA-256: 09f015126c80383e8f025c545c45d1cb9224e72c367818badd48586b21208072
```

使用 Apple Development 身份签名，未公证，仅用于本机/内部测试。

### Windows x64

```text
dist/windows/VibeCodingRemote-0.1.2-windows-x64-portable.zip
SHA-256: 97dfd803d87c1b9385da572ce0fd23df0f862f32218a5ceeae054db09b33eb7a
```

Portable ZIP 内为 GUI 子系统 x64 PE、`mobile-web/` 手机页面资源和说明文件，静态链接 Microsoft C/C++ Runtime。当前未做 Authenticode 签名，SmartScreen 可能提示。需要 Microsoft Edge WebView2 Runtime。

## 共同测试

1. 启动后显示桌面管理窗口；
2. 状态卡能显示 Server、输入能力和手机连接状态；
3. 修改端口并点击“保存并重启服务”；
4. 新端口 `/health` 返回 `ok`；
5. 点击“复制地址”；
6. 点击“复制完整配置”，确认 JSON 包含 endpoint 与 token，但页面不直接显示 Token；
7. 手机扫描二维码或导入配置；二维码内容必须是 `vibecodingremote://import?...`，可被现有手机扫码逻辑直接识别；
8. 手机直接打开管理页给出的局域网根地址，确认本地 Mobile Web、CSS 和 JS 均正常加载；
9. 手机发送中文、多行文字、Enter、Escape、Backspace、方向键；
10. 关闭管理窗口后 Server 仍在线；
11. 从托盘/菜单栏重新打开窗口；
12. 从托盘/菜单栏显式退出后 Server 停止。

## macOS 专项

1. 将 App 拖入 `/Applications`；
2. 在管理页确认“辅助功能”状态；
3. 未授权时点击权限按钮；
4. 授权正式 `Vibe Coding Remote.app` 后完全退出并重开；
5. TextEdit 中文粘贴；
6. Enter 换行后再次粘贴；
7. Terminal/Pi 中测试 Enter、Escape、Ctrl+C；
8. 确认菜单栏图标可打开和退出。

## Windows 专项

1. 解压 ZIP 后运行 `VibeCodingRemote.exe`；
2. SmartScreen 出现时确认文件 SHA-256 后选择继续；
3. 如 Windows 防火墙提示，仅允许受信任的专用网络；
4. 检查系统托盘图标；
5. 在记事本测试中文粘贴、Enter、Backspace 和方向键；
6. 在 Windows Terminal/Pi 测试 Enter、Escape、Ctrl+C；
7. 验证普通权限 App 可以控制普通窗口；
8. 验证管理员窗口被 UIPI 阻止时，管理页和普通窗口能力不受影响；
9. 关闭窗口后检查托盘常驻；
10. 托盘退出后检查端口释放。

## 已完成的构建验证

- `cargo clippy`：Desktop/Server `-D warnings` 通过；
- `cargo test --workspace`：6 项测试通过；
- Windows release PE 构建成功；
- Windows 测试目标 `cargo xwin test --no-run` 成功；
- Windows PE GUI subsystem、图标资源和静态 CRT 已检查；
- macOS `.app` / `.dmg` 构建成功；
- DMG 内 App 签名验证成功；
- macOS App 启动后内嵌 Server `/health` 返回 `ok`；
- Desktop App 已打包 Mobile Web，根页面及 CSS/JS 静态资源 smoke 均返回 HTTP 200；
- Server 日志不再记录 Token、导入 URL、二维码或用户输入正文。

## 尚需现场确认

- Windows WebView2 启动与托盘行为；
- Windows 防火墙提示；
- Windows `SendInput` 真实焦点输入；
- 新 Tauri macOS App 身份的辅助功能授权与真实输入；
- 手机浏览器对两个平台的完整端到端控制。

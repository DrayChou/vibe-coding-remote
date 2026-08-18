# Vibe Coding Remote Desktop

Windows 与 macOS 共用的 Tauri 2 管理端。

## 职责

- 同进程启动 `vibe-coding-remote-server`；
- 即使 Server 端口冲突或启动失败，管理窗口仍保持可用；
- 展示服务状态、最近错误、局域网地址和平台输入权限；
- 保存监听地址、端口和私密 Token；
- 将 Mobile Web 一起打包并由同一个 Rust Server 提供；
- 生成与手机扫码解析器一致的 `vibecodingremote://import` 配置二维码；
- 提供 Windows 托盘 / macOS 菜单栏入口；
- 关闭窗口时隐藏到托盘，显式“退出”才结束进程。

## 开发

```bash
pnpm install
pnpm run dev:desktop
```

## 构建

```bash
pnpm run build:desktop
```

- macOS：生成 `.app` 与 `.dmg`；
- Windows：在 Windows Runner 上生成对应安装产物；macOS 可使用 `cargo-xwin` 生成 x64 Portable EXE，但正式安装器仍应由 Windows Runner 产出。

## 配置

配置由 Tauri 的应用配置目录管理，包含监听地址、端口和控制 Token。Unix 平台目录权限为 `0700`、配置文件为 `0600`。管理页默认不显示 Token 明文，仅在用户明确点击“复制完整配置”或展示连接二维码时使用。

## 平台边界

- macOS 输入注入需要为正式签名的 `Vibe Coding Remote.app` 授予“辅助功能”权限；
- Windows 使用 `SendInput`，不能向完整性级别更高的管理员窗口注入；
- Windows 防火墙自动配置、登录项和自动更新尚未实现，应在后续阶段补齐。

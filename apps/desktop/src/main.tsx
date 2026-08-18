import React from 'react';
import ReactDOM from 'react-dom/client';
import { invoke } from '@tauri-apps/api/core';
import { CheckCircle2, CircleAlert, Copy, KeyRound, Network, Play, RefreshCw, ShieldCheck } from 'lucide-react';
import './styles.css';

type DesktopStatus = {
  running: boolean;
  last_error: string | null;
  host: string;
  port: number;
  config_path: string;
  platform: string;
  input_permission: 'granted' | 'denied' | 'not-required';
  lan_urls: string[];
  token_present: boolean;
  mobile_web_ready: boolean;
};

type ConnectionConfig = {
  endpoint: string;
  token: string;
  import_url: string;
};

function App() {
  const [status, setStatus] = React.useState<DesktopStatus | null>(null);
  const [busy, setBusy] = React.useState(false);
  const [message, setMessage] = React.useState<string | null>(null);
  const [host, setHost] = React.useState('0.0.0.0');
  const [port, setPort] = React.useState(8765);
  const [qrSvg, setQrSvg] = React.useState('');

  const refresh = React.useCallback(async () => {
    const next = await invoke<DesktopStatus>('get_desktop_status');
    setStatus(next);
    setHost(next.host);
    setPort(next.port);
    const svg = await invoke<string>('get_connection_qr_svg');
    setQrSvg(svg);
  }, []);

  React.useEffect(() => {
    refresh().catch((error) => setMessage(String(error)));
    const timer = window.setInterval(() => refresh().catch(() => undefined), 3000);
    return () => window.clearInterval(timer);
  }, [refresh]);

  async function run(label: string, operation: () => Promise<unknown>) {
    setBusy(true);
    setMessage(null);
    try {
      await operation();
      setMessage(label);
      await refresh();
    } catch (error) {
      setMessage(String(error));
    } finally {
      setBusy(false);
    }
  }

  if (!status) {
    return <main className="loading">正在读取 Vibe Coding Remote 状态…</main>;
  }

  const primaryUrl = status.lan_urls[0] ?? `http://127.0.0.1:${status.port}`;

  return (
    <main className="shell">
      <header className="hero">
        <div>
          <p className="eyebrow">VIBE CODING REMOTE</p>
          <h1>桌面控制中心</h1>
          <p className="subtitle">管理手机连接、系统权限和 Windows / macOS 输入服务。</p>
        </div>
        <button className="icon-button" onClick={() => refresh()} title="刷新状态">
          <RefreshCw size={18} />
        </button>
      </header>

      <section className="status-grid">
        <StatusCard
          title="本地服务"
          value={status.running ? '运行中' : '未运行'}
          detail={status.last_error ?? `${status.host}:${status.port}`}
          good={status.running}
          icon={<Play size={20} />}
        />
        <StatusCard
          title="输入权限"
          value={status.input_permission === 'granted' ? '已授权' : status.input_permission === 'not-required' ? '无需授权' : '需要处理'}
          detail={status.platform === 'macos' ? 'macOS 辅助功能' : 'Windows SendInput'}
          good={status.input_permission !== 'denied'}
          icon={<ShieldCheck size={20} />}
        />
        <StatusCard
          title="手机连接"
          value={status.mobile_web_ready ? (status.lan_urls.length > 0 ? '局域网可访问' : '仅本机') : '手机页面缺失'}
          detail={status.mobile_web_ready ? primaryUrl : '重新构建 Desktop App'}
          good={status.mobile_web_ready && status.lan_urls.length > 0}
          icon={<Network size={20} />}
        />
      </section>

      <section className="panel">
        <div className="panel-heading">
          <div>
            <h2>手机连接</h2>
            <p>在手机 Web 页面中填写服务器地址和 Token。Token 默认不会在管理页明文展示。</p>
          </div>
          <div className="heading-actions">
            <button
              className="secondary"
              onClick={() => navigator.clipboard.writeText(primaryUrl).then(() => setMessage('连接地址已复制'))}
            >
              <Copy size={16} />复制地址
            </button>
            <button
              className="secondary"
              onClick={() => run('包含 Token 的完整配置已复制，请仅发送到自己的手机', async () => {
                const config = await invoke<ConnectionConfig>('get_connection_config');
                await navigator.clipboard.writeText(config.import_url);
              })}
            >
              <KeyRound size={16} />复制完整配置
            </button>
          </div>
        </div>
        <div className="connection-layout">
          <div className="connection-copy">
            <div className="url-box">{primaryUrl}</div>
            <div className="inline-note"><KeyRound size={15} /> Token 已生成：{status.token_present ? '是' : '否'}</div>
            <p className="security-note">二维码和完整配置包含控制 Token，只应由你自己的手机扫描或保存。</p>
          </div>
          {qrSvg && <div className="qr-code" aria-label="手机连接配置二维码" dangerouslySetInnerHTML={{ __html: qrSvg }} />}
        </div>
      </section>

      <section className="panel">
        <div className="panel-heading">
          <div>
            <h2>服务设置</h2>
            <p>修改后会保存到私密配置文件，并重启内嵌 Server。</p>
          </div>
        </div>
        <div className="form-grid">
          <label>
            <span>监听地址</span>
            <select value={host} onChange={(event) => setHost(event.target.value)}>
              <option value="0.0.0.0">允许局域网手机连接</option>
              <option value="127.0.0.1">仅允许本机</option>
            </select>
          </label>
          <label>
            <span>端口</span>
            <input type="number" min={1024} max={65535} value={port} onChange={(event) => setPort(Number(event.target.value))} />
          </label>
        </div>
        <div className="actions">
          <button
            className="primary"
            disabled={busy}
            onClick={() => run('设置已保存，服务已重新启动', () => invoke('save_and_restart_server', { host, port }))}
          >
            保存并重启服务
          </button>
          {status.platform === 'macos' && status.input_permission !== 'granted' && (
            <button
              className="secondary"
              disabled={busy}
              onClick={() => run('已请求辅助功能权限', () => invoke('request_input_permission'))}
            >
              打开辅助功能授权
            </button>
          )}
        </div>
      </section>

      <section className="panel compact">
        <h2>诊断信息</h2>
        <dl>
          <div><dt>平台</dt><dd>{status.platform}</dd></div>
          <div><dt>配置文件</dt><dd>{status.config_path}</dd></div>
          <div><dt>最近错误</dt><dd>{status.last_error ?? '无'}</dd></div>
        </dl>
      </section>

      {message && <div className="toast">{message}</div>}
    </main>
  );
}

function StatusCard(props: { title: string; value: string; detail: string; good: boolean; icon: React.ReactNode }) {
  return (
    <article className={`status-card ${props.good ? 'good' : 'warn'}`}>
      <div className="status-icon">{props.icon}</div>
      <div>
        <p>{props.title}</p>
        <strong>{props.value}</strong>
        <small>{props.detail}</small>
      </div>
      {props.good ? <CheckCircle2 size={18} /> : <CircleAlert size={18} />}
    </article>
  );
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode><App /></React.StrictMode>,
);

# Camellia Server

Camellia's self-hosted ID/rendezvous and relay servers, a fork of
[rustdesk-server](https://github.com/rustdesk/rustdesk-server).

## Binaries

```bash
cargo build --release
```

Three executables are generated in `target/release`:

- `hbbs` — ID/rendezvous server (default port 21116)
- `hbbr` — relay server (default port 21117)
- `rustdesk-utils` — CLI utilities (`genkeypair`, `validatekeypair`, `doctor`)

## Listeners

| Service | Port | Notes |
|---|---|---|
| hbbs NAT test | 21115 (ID port - 1) | TCP |
| hbbs ID/rendezvous | 21116 (ID port) | TCP + UDP |
| hbbr relay | 21117 (relay port) | TCP |
| hbbs WebSocket | 21118 (ID port + 2) | for WS clients |
| hbbr WebSocket | 21119 (relay port + 2, i.e. ID port + 3) | for WS clients |

Both binaries accept `-b/--bind <IP>` (`BIND`) to listen on a single
interface instead of all of them.

## Options

### `--trust-proxy-headers` (hbbs and hbbr)

When set to `Y` (flag or `TRUST_PROXY_HEADERS` env var), the WebSocket
listeners take the client address from the `X-Real-IP` /
`X-Forwarded-For` headers instead of the TCP source address.

Only enable this behind a trusted reverse proxy that overwrites those
headers; otherwise any client can spoof its IP and bypass IP-based
blocking/rate limiting.

### `--api-server <URL>` (hbbs)

Base URL of the HTTP API server backing the built-in API proxy. Clients
send `HttpProxyRequest` messages over the rendezvous channel, and hbbs
forwards them (paths under `/api/` and `/lic/web/api/` only) to this
server. Defaults to `API_SERVER` env var or `http://127.0.0.1:<ID port - 2>`.

### Keys

Run with `-k <key>` to restrict access to clients that present the same
key. The key pair is read from/generated to `id_ed25519`(`.pub`) in the
working directory; `rustdesk-utils genkeypair` creates one manually.

Env vars can also be provided via a `.env` file (INI format) in the
working directory. See
**[docs/environment-variables.md](docs/environment-variables.md)** for the
full variable list, precedence rules, and tuning options.

## License and attribution

Fork of [RustDesk Server OSS](https://github.com/rustdesk/rustdesk-server)
by Purslane Ltd., used under the [AGPL-3.0](LICENSE) license. This fork
(Camellia) remains licensed under AGPL-3.0. Upstream documentation:
<https://rustdesk.com/docs/en/self-host/>.

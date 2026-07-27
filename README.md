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
- `rustdesk-utils` — CLI utilities (`genkeypair`, `validatekeypair`, `doctor`,
  `healthcheck`)

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
send `HttpProxyRequest` messages over a server-authenticated,
application-encrypted
rendezvous channel, and hbbs forwards a bounded allowlist of requests (paths
under `/api/` and `/lic/web/api/` only) to this server. The value must be a
bare HTTP(S) origin, without credentials, path, query, or fragment. Remote
origins must use HTTPS; plain HTTP is accepted only for loopback. It defaults to
`API_SERVER` or `http://127.0.0.1:<ID port - 2>`.

### Keys

Both services require a key. With no explicit value, `-`, or `_`, they load or
atomically generate `id_ed25519` and `id_ed25519.pub` in the working directory.
An explicit `hbbs` key must be the structurally valid 64-byte Ed25519 private
key because the rendezvous service signs its secure handshake. `hbbr` may use
that private key or its 32-byte public half.
The private key is restricted to mode `0600` on Unix and malformed or
unwritable key material stops startup instead of silently opening the service.
`rustdesk-utils genkeypair` can create a pair manually.

Env vars can also be provided via a `.env` file (INI format) in the
working directory. See
**[docs/environment-variables.md](docs/environment-variables.md)** for the
full variable list, precedence rules, and tuning options.

## Production Compose

The included `docker-compose.yml` deliberately has no `latest` fallback. Set
an immutable Camellia classic-image tag or digest and the externally reachable
relay address. Managed mode also requires the API origin and the same
high-entropy device-verification secret configured on the API server:

```bash
export CAMELLIA_SERVER_IMAGE='ghcr.io/xavieralpha/rustdesk-server@sha256:<digest>'
export CAMELLIA_RELAY_ADDRESS='rustdesk.example.com:21117'
export CAMELLIA_API_SERVER='https://api.example.com'
export CAMELLIA_DEVICE_VERIFICATION_TOKEN_FILE='/secure/device-verification-token'
docker compose up -d
```

The deployment drops Linux capabilities, enables `no-new-privileges`, uses a
read-only root filesystem, bounds PIDs and log retention, and health-checks
both listeners. Persistent state is held in the `camellia-server-data` named
volume, whose initial ownership matches the image's unprivileged UID/GID
`10001`. Set `CAMELLIA_BIND_IP` to bind published ports to a specific host
address. The Compose model intentionally targets the classic image; the s6
image has its own supervisor entrypoint and should be deployed as one
container instead.

`kubernetes/example.yaml` provides the equivalent hardened, single-replica
Kubernetes baseline. Replace its image digest and `.invalid` endpoints, create
the referenced verification-token Secret, and confirm that the cluster's
LoadBalancer implementation supports TCP and UDP ports in one Service. The
single replica and `Recreate` strategy are intentional because hbbs uses local
SQLite state and both processes share one `ReadWriteOnce` volume.

Release archives contain the three native binaries for Linux and Windows. The
former Windows GUI/NSIS bundle was retired because it installed an embedded,
unverifiable service wrapper as `LocalSystem`; it is not a production-safe
service boundary.

## License and attribution

Fork of [RustDesk Server OSS](https://github.com/rustdesk/rustdesk-server)
by Purslane Ltd., used under the [AGPL-3.0](LICENSE) license. This fork
(Camellia) remains licensed under AGPL-3.0. Upstream documentation:
<https://rustdesk.com/docs/en/self-host/>.

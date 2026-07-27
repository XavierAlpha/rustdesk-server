# Configuration & Environment Variables

This document is the single reference for every option that the open‑source
RustDesk server binaries (`hbbs`, `hbbr`) understand: command‑line flags,
environment variables, and configuration files.

> **TL;DR** — For most people the command‑line flags shown by `hbbs --help` /
> `hbbr --help` are all you need. Environment variables are an alternative way to
> set the same options, plus a handful of extra tuning knobs that have no flag.

---

## How configuration is loaded

Both servers read their configuration from the following sources. For
**`hbbs`** the order of precedence, from highest to lowest, is:

1. **Command‑line flag** (e.g. `-p 21116`, `-k mykey`)
2. **`--config <file>`** — an INI file passed with `-c`/`--config`
3. **`.env`** — an INI file named `.env` in the working directory
4. **Inherited process environment** — variables exported before launch

A value set by a higher source overrides the same value from a lower one. Under
the hood every source is turned into a process environment variable, and the
code then reads that variable — so "flag", "config file" and "env var" are just
three ways to set the same thing.

For **`hbbr`** the precedence is: **flag** (`-b`, `-p`, `-k`) → **`.env`** →
**inherited environment**.

Configuration files must be UTF-8 regular files no larger than 1 MiB.
Malformed, oversized, unreadable, or non-regular files stop startup; Unix
symbolic links are rejected. A missing default `.env` is the only ignored
file condition, while an explicitly requested `--config` file must exist.

`RUST_LOG` is an exception to these rules. Both binaries initialize logging
before loading `.env` (or `hbbs`'s `--config` file), so `RUST_LOG` must be set
in the inherited process environment.

---

## `hbbs` — ID / rendezvous server

| Variable | CLI flag | Default | Description |
|---|---|---|---|
| `KEY` | `-k`, `--key` | `-` | A structurally valid base64 64-byte Ed25519 private key, or an empty value / `-` / `_` to load or atomically generate a key pair (`id_ed25519`, `id_ed25519.pub`). `hbbs` needs the private key to sign the secure rendezvous handshake; public-only values are rejected. Key validation cannot be disabled. |
| `BIND` | `-b`, `--bind` | all interfaces | **Available since 1.1.17.** Local IPv4 or IPv6 address on which all `hbbs` TCP, UDP, and WebSocket listeners bind. This does not change the addresses advertised to clients. Supported by `--config`, `.env`, and the inherited environment. |
| `API_SERVER` | `--api-server` | `http://127.0.0.1:<HBBS_PORT-2>` | Bare HTTP(S) origin for the built-in API gateway. Remote origins must use HTTPS; HTTP is accepted only for loopback. Credentials, paths, queries, and fragments are rejected. The gateway requires a secure rendezvous handshake and matching server key, canonicalizes allowed API paths, overwrites forwarded client-IP headers, and enforces bounded headers/body/concurrency/rate. |
| `DEVICE_VERIFICATION_TOKEN` 🅴 | *(none)* | *(none)* | 32–512 character secret shared only with the API server for device-deployment verification. Required unless unmanaged mode is explicitly enabled. This value is read only from the inherited process environment; do not place it in flags or configuration files. |
| `DEVICE_VERIFICATION_TOKEN_FILE` 🅴 | *(none)* | *(none)* | Path to a bounded regular file containing the verification secret; preferred for containers. Symbolic links are rejected on Unix. Kubernetes Secret keys must therefore be mounted as a single-file `subPath`, as in the included manifest. Mutually exclusive with `DEVICE_VERIFICATION_TOKEN`. |
| `ALLOW_UNMANAGED_DEVICES` 🅴 | *(none)* | `N` | `Y` lets first-seen devices claim IDs without API approval. This weakens the managed deployment boundary and must be an explicit operator decision. |
| `HBBS_PORT` | `-p`, `--port` | `21116` | Main TCP/UDP listening port. `hbbs` also binds `HBBS_PORT-1` (NAT type test) and `HBBS_PORT+2` (WebSocket). |
| `RELAY-SERVERS` | `-r`, `--relay-servers` | *(empty)* | Optional relay server override handed to clients, as comma-separated `host` or `host:port` values. Leave empty when `hbbr` uses the same address as `hbbs` and the standard port `21117`; clients derive it automatically. Set this only when the relay uses a different IP/hostname or a non-standard port. |
| `RMEM` | `-M`, `--rmem` | `0` (system default) | UDP receive‑buffer size in bytes. Raise the OS limit first: `sudo sysctl -w net.core.rmem_max=52428800`. |
| *(config file)* | `-c`, `--config` | *(none)* | Path to an extra INI config file (see precedence above). |
| `ALWAYS_USE_RELAY` 🅴 | *(none)* | `N` | `Y` forces every session through a relay (disables direct/hole‑punched connections). At runtime, send `always-use-relay Y` or `always-use-relay N` to the `hbbs` [loopback console](#runtime-console). |
| `DB_URL` 🅴 | *(none)* | `./db_v2.sqlite3` | Path/URL of the SQLite database file. See [Database](#database). |
| `MAX_DATABASE_CONNECTIONS` 🅴 | *(none)* | `1` | Size of the SQLite connection pool; must be 1–32 or startup fails. Connections use WAL mode, full synchronous durability, foreign-key enforcement, and a bounded busy timeout. |
| `MAX_RENDEZVOUS_CONNECTIONS` 🅴 | *(none)* | `4096` | Maximum concurrent TCP/WebSocket rendezvous connections; must be 64–65536. |
| `MAX_CACHED_PEERS` 🅴 | *(none)* | `100000` | Maximum in-memory peer cache entries; must be 1024–1000000. Inactive entries are evicted periodically. |
| `ENABLE_RUNTIME_CONSOLE` 🅴 | *(none)* | `N` | `Y` enables the unauthenticated loopback TCP console. Leave disabled in production; see [Runtime console](#runtime-console). |

🅴 = no command-line flag. Except for the direct secret called out above,
these values may be set through `--config`, `.env`, or the inherited process
environment.

> `PORT_FOR_API` / `KEY_FOR_API` are only used by RustDesk Server **Pro** and its
> API; they have no effect in the open‑source server.

---

## `hbbr` — relay server

| Variable | CLI flag | Default | Description |
|---|---|---|---|
| `KEY` | `-k`, `--key` | *(empty)* | An empty value, `-`, or `_` loads or atomically generates the shared key pair. A public or base64 secret key selects that explicit key. Relay key validation cannot be disabled. |
| `BIND` | `-b`, `--bind` | all interfaces | **Available since 1.1.17.** Local IPv4 or IPv6 address on which the relay TCP and WebSocket listeners bind. Supported by `.env` and the inherited environment; `hbbr` does not support `--config`. |
| `HBBR_PORT` | `-p`, `--port` | `21117` | Relay listening port. The environment variable and CLI flag have identical semantics; `hbbr` also binds `HBBR_PORT+2` for WebSocket relay. |
| `MAX_RELAY_CONNECTIONS` | *(none)* | `8192` | Maximum concurrent relay TCP/WebSocket connections; must be 64–65536. Pending relay pairing is additionally capped globally and per source IP. |
| `ENABLE_RUNTIME_CONSOLE` | *(none)* | `N` | `Y` enables the unauthenticated loopback TCP console. |

### Relay bandwidth / QoS

These have no CLI flag and can also be changed through the `hbbr`
[loopback console](#runtime-console) (`tb`, `sb`, `ls`, `dt`, `t`, …; send `h`
for help).

Configured values are validated at startup. The downgrade threshold must be in
the range `0.000001`–`1`, all durations and bandwidths must be positive, and
`LIMIT_SPEED` cannot exceed `SINGLE_BANDWIDTH`.

| Variable | Default | Unit | Description |
|---|---|---|---|
| `SINGLE_BANDWIDTH` | `128` | Mb/s | Normal maximum bandwidth for each relay connection. |
| `TOTAL_BANDWIDTH` | `1024` | Mb/s | Aggregate bandwidth cap shared by all relay connections. |
| `LIMIT_SPEED` | `32` | Mb/s | Per-connection cap applied after a connection is downgraded, and to IPs in `blacklist.txt`. |
| `DOWNGRADE_THRESHOLD` | `0.66` | ratio (0.000001–1) | Fraction of `SINGLE_BANDWIDTH` that a connection's lifetime-average throughput must exceed to trigger downgrade. Values use six-decimal precision. |
| `DOWNGRADE_START_CHECK` | `1800` | seconds | Delay before a connection becomes eligible for the lifetime-average downgrade check. |

Downgrade is decided independently for each connection; it does **not** check
aggregate relay congestion. After `DOWNGRADE_START_CHECK`, a connection is
capped to `LIMIT_SPEED` once its average throughput since it started exceeds
`SINGLE_BANDWIDTH * DOWNGRADE_THRESHOLD`. A lone transfer can therefore be
downgraded even when the relay is otherwise idle. `TOTAL_BANDWIDTH` is a
separate aggregate cap.

These may also be placed in `.env` using the uppercase spellings shown above
(e.g. `SINGLE_BANDWIDTH=256`).

### Blocklists / blacklists (files, not env vars)

`hbbr` reads two optional files from its working directory at start‑up:

* **`blacklist.txt`** — IPs that are **bandwidth‑limited** (one IP per line;
  anything after the first space on a line is ignored).
* **`blocklist.txt`** — IPs that are **refused** outright.

Both can also be edited live through the `hbbr` loopback console (`ba`/`br`,
`Ba`/`Br`). Files are limited to 1 MiB and 65,536 canonical IP entries. Blank
lines and `#` comments are accepted; an unreadable file, invalid first token,
invalid UTF-8, or an exceeded limit stops startup rather than silently
discarding policy.

### Runtime console

The runtime consoles are disabled by default because any local process could
otherwise mutate live server policy without authentication. They are TCP
command transports, not `rustdesk-utils` commands or interactive
standard-input consoles. Only after explicitly setting
`ENABLE_RUNTIME_CONSOLE=Y` is a loopback connection treated as one command:

```bash
# hbbs: toggle forced relay on HBBS_PORT-1 (21115 by default)
printf 'always-use-relay Y' | nc 127.0.0.1 21115

# hbbr: list commands on HBBR_PORT (21117 by default)
printf 'h' | nc 127.0.0.1 21117
```

Use the corresponding configured ports if you changed `HBBS_PORT` or
`HBBR_PORT`.

---

## Database

At runtime the database location comes from **`DB_URL`** (default
`./db_v2.sqlite3`). If unset, `hbbs` creates the SQLite file in its working
directory. Builds do not require a pre-created database or a compile-time
`DATABASE_URL`.

---

## Logging

Both binaries use `flexi_logger`, which honours the standard **`RUST_LOG`**
environment variable (default level `info`). Set it in the process environment
before launching the binary. A value in `.env` or `hbbs`'s `--config` file is
loaded too late and has no effect on logging.

```bash
RUST_LOG=debug hbbs
```

---

## Keys and encryption

For `hbbr`, the `KEY` / `-k` value can be:

* a **base64-encoded 32-byte Ed25519 public key** — clients must present the
  matching key;
* a **structurally valid base64-encoded 64-byte Ed25519 private key** — the
  server derives the public key from it;
* **`-` or `_`** — the server loads a key pair from the working directory or
  generates one on first start, writing `id_ed25519` (private) and
  `id_ed25519.pub` (public);
* **empty** — the same safe load-or-generate behavior as `-` / `_`; it does not
  disable validation.

`hbbs` accepts the load-or-generate values above or a structurally valid
base64-encoded 64-byte Ed25519 private key; it deliberately rejects public-only
configuration because it signs the secure rendezvous handshake. `hbbs`
defaults to `-`; `hbbr` defaults to the equivalent empty value. Both therefore
load or generate keys and enforce validation without extra flags.
Run both services from the same working directory (or mount the same key
volume) so they use identical material. Key creation is atomic across
concurrent starts, the private file is forced to `0600` on Unix, and malformed
or unwritable material fails startup.

To supply your own key pair, place `id_ed25519` and `id_ed25519.pub` in the
process's **current working directory** before first start. That directory may
differ from the directory containing the executable. For the supervisor Docker
image, the working directory is `/data`.

---

## Docker image variables

The supervisor image (`ghcr.io/xavieralpha/rustdesk-server-s6`) starts both
binaries as the unprivileged `nobody` account under a root s6 supervisor. It
adds a few convenience variables handled by its service scripts, **not** by
`hbbs`/`hbbr` directly:

| Variable | Default | Description |
|---|---|---|
| `RELAY` | *(required)* | Passed to `hbbs` as `-r $RELAY`. Startup fails unless this is the externally reachable relay `host:port`. |
| `S6_READ_ONLY_ROOT` | `0` | Set to `1` when the container uses a read-only root filesystem, and mount a writable tmpfs at `/run`. |
| Docker secret `key_pub` | *(unset)* | If mounted at `/run/secrets/key_pub`, copied to `/data/id_ed25519.pub` on first start. |
| Docker secret `key_priv` | *(unset)* | If mounted at `/run/secrets/key_priv`, copied atomically to `/data/id_ed25519` on first start. The public half is derived when `key_pub` is omitted. A public secret without a private secret is rejected. Private keys are intentionally not accepted through environment variables or process arguments. |

Any variable from the tables above can also be passed straight through the
container's environment (e.g. `-e ALWAYS_USE_RELAY=Y`, `-e RUST_LOG=debug`).

The classic scratch image (`ghcr.io/xavieralpha/rustdesk-server`) contains only
the binaries, runs as UID/GID `10001`, and uses `/data` as its working
directory. It does **not** implement the supervisor conveniences or Docker
secret bootstrap.

---

## Examples

### Command line — non-standard ports

```bash
# Tell clients where the relay listens because it is not using port 21117.
hbbs -p 22116 -r rustdesk.example.com:22117
hbbr -p 22117
```

### `.env` file (working directory)

```ini
# Non-standard ports shared through one working-directory configuration.
relay-servers=rustdesk.example.com:22117
HBBS_PORT=22116
HBBR_PORT=22117
```

### docker-compose

```yaml
services:
  rustdesk-server:
    image: ghcr.io/xavieralpha/rustdesk-server-s6@sha256:<digest>
    environment:
      - RELAY=rustdesk.example.com:21117
      - API_SERVER=https://api.example.com
      - DEVICE_VERIFICATION_TOKEN_FILE=/run/secrets/device_verification_token
      - ALWAYS_USE_RELAY=Y
      - RUST_LOG=info
      - S6_READ_ONLY_ROOT=1
      - SINGLE_BANDWIDTH=256
    secrets:
      - device_verification_token
    ports:
      - "21115:21115"
      - "21116:21116"
      - "21116:21116/udp"
      - "21117:21117"
      - "21118:21118"
      - "21119:21119"
    volumes: ["camellia-data:/data"]
    read_only: true
    tmpfs:
      - /run:size=16m,mode=0755
      - /tmp:size=16m,mode=1777
    restart: unless-stopped

volumes:
  camellia-data:

secrets:
  device_verification_token:
    file: /secure/device-verification-token
```

### systemd

The Debian units run as the dedicated `camellia-server` user and read
`/etc/default/camellia-server`. `rustdesk-hbbs.service` is deliberately skipped
until that file exists and is non-empty, so a package install cannot expose an
unconfigured managed-device endpoint. Start from the packaged example and
replace every `.invalid` value:

```ini
API_SERVER=https://api.example.com
DEVICE_VERIFICATION_TOKEN_FILE=/etc/camellia-server/device-verification-token
RELAY_SERVERS=rustdesk.example.com:21117
ALWAYS_USE_RELAY=Y
RUST_LOG=info
```

Store the token file as `root:camellia-server` with mode `0640`. Both services
share `/var/lib/rustdesk-server`; package removal deliberately preserves this
directory so uninstalling one service cannot erase the other's key material.
After validating the API origin, relay address, and shared secret, enable and
start both units:

```bash
sudo systemctl enable --now rustdesk-hbbs.service rustdesk-hbbr.service
```

---

## Port reference

| Port | Proto | Server | Purpose |
|---|---|---|---|
| 21115 | TCP | hbbs | NAT type test (`HBBS_PORT-1`) |
| 21116 | TCP + UDP | hbbs | ID registration / rendezvous / hole punching (`HBBS_PORT`) |
| 21117 | TCP | hbbr | Relay (`HBBR_PORT`) |
| 21118 | TCP | hbbs | WebSocket rendezvous (`HBBS_PORT+2`) |
| 21119 | TCP | hbbr | WebSocket relay (`HBBR_PORT+2`) |

Ports 21118/21119 are only needed for the web client; you can omit them
otherwise.

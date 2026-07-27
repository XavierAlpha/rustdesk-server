Source: rustdesk-server
Section: net
Priority: optional
Maintainer: XavierAlpha <52537840+XavierAlpha@users.noreply.github.com>
Build-Depends: debhelper-compat (= 13)
Standards-Version: 4.7.2
Rules-Requires-Root: no
Homepage: https://github.com/XavierAlpha/rustdesk-server

Package: rustdesk-server-hbbs
Architecture: {{ ARCH }}
Depends: adduser, systemd, ${misc:Depends}, ${shlibs:Depends}
Description: Camellia rendezvous server
 Self-hosted Camellia device discovery and rendezvous service.

Package: rustdesk-server-hbbr
Architecture: {{ ARCH }}
Depends: adduser, systemd, ${misc:Depends}, ${shlibs:Depends}
Description: Camellia relay server
 Self-hosted Camellia encrypted connection relay service.

Package: rustdesk-server-utils
Architecture: {{ ARCH }}
Depends: ${misc:Depends}, ${shlibs:Depends}
Description: Camellia server utilities
 Diagnostic and key-management utilities for Camellia servers.

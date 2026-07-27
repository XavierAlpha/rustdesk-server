#!/bin/sh

set -eu

/usr/bin/rustdesk-utils healthcheck 127.0.0.1:21117
/usr/bin/rustdesk-utils healthcheck 127.0.0.1:21116

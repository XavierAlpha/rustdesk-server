FROM busybox:1.38.0-musl@sha256:8635836765b0c4c43970660219739baa58b0883c2e429e4b8918f7dd1519455c AS rootfs
RUN mkdir -p /data && chown 10001:10001 /data && chmod 0700 /data

FROM scratch
COPY --from=rootfs --chown=10001:10001 /data /data
COPY hbbs /usr/bin/hbbs
COPY hbbr /usr/bin/hbbr
COPY rustdesk-utils /usr/bin/rustdesk-utils
USER 10001:10001
WORKDIR /data
ENV HOME=/data
